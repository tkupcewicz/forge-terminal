use crate::termwindow::box_model::*;
use crate::termwindow::render::corners::*;
use crate::termwindow::render::window_buttons::window_button_element;
use crate::termwindow::{UIItem, UIItemType};
use crate::utilsprites::RenderMetrics;
use config::{Dimension, DimensionContext, TabBarColors};
use mux::Mux;
use wezterm_term::color::ColorPalette;
use window::color::LinearRgba;
use window::{IntegratedTitleButton, WindowDecorations, WindowState};

impl crate::TermWindow {
    pub fn build_side_panel(&self, _palette: &ColorPalette) -> anyhow::Result<ComputedElement> {
        let font = self.fonts.title_font()?;
        let metrics = RenderMetrics::with_font_metrics(&font.metrics());

        let _colors = self
            .config
            .colors
            .as_ref()
            .and_then(|c| c.tab_bar.as_ref())
            .cloned()
            .unwrap_or_else(TabBarColors::default);

        let bar_colors = ElementColors {
            border: BorderColor::default(),
            bg: if self.focused.is_some() {
                self.config.window_frame.active_titlebar_bg
            } else {
                self.config.window_frame.inactive_titlebar_bg
            }
            .to_linear()
            .into(),
            text: if self.focused.is_some() {
                self.config.window_frame.active_titlebar_fg
            } else {
                self.config.window_frame.inactive_titlebar_fg
            }
            .to_linear()
            .into(),
        };

        // Dimmer text color for non-highlighted elements (60% fg + 40% bg)
        let dim_text: InheritableColor = {
            let base = if self.focused.is_some() {
                self.config.window_frame.active_titlebar_fg
            } else {
                self.config.window_frame.inactive_titlebar_fg
            }
            .to_linear();
            let bg = if self.focused.is_some() {
                self.config.window_frame.active_titlebar_bg
            } else {
                self.config.window_frame.inactive_titlebar_bg
            }
            .to_linear();
            LinearRgba::with_components(
                base.0 * 0.6 + bg.0 * 0.4,
                base.1 * 0.6 + bg.1 * 0.4,
                base.2 * 0.6 + bg.2 * 0.4,
                base.3,
            )
            .into()
        };

        // --- Additional color palette ---

        // Warm amber accent color (~#c4a77d)
        let accent_amber = LinearRgba::with_components(0.769, 0.655, 0.490, 1.0);

        // Separator: fg at 12% alpha
        let separator = {
            let fg_lin = if self.focused.is_some() {
                self.config.window_frame.active_titlebar_fg
            } else {
                self.config.window_frame.inactive_titlebar_fg
            }
            .to_linear();
            LinearRgba::with_components(fg_lin.0, fg_lin.1, fg_lin.2, 0.12)
        };

        // Compute bg/fg linear once for reuse
        let bg_lin = if self.focused.is_some() {
            self.config.window_frame.active_titlebar_bg
        } else {
            self.config.window_frame.inactive_titlebar_bg
        }
        .to_linear();
        let fg_lin = if self.focused.is_some() {
            self.config.window_frame.active_titlebar_fg
        } else {
            self.config.window_frame.inactive_titlebar_fg
        }
        .to_linear();

        // Elevated background: 88% bg + 12% fg
        let elevated_bg: InheritableColor = LinearRgba::with_components(
            bg_lin.0 * 0.88 + fg_lin.0 * 0.12,
            bg_lin.1 * 0.88 + fg_lin.1 * 0.12,
            bg_lin.2 * 0.88 + fg_lin.2 * 0.12,
            bg_lin.3,
        )
        .into();

        // Pill background: 92% bg + 8% fg
        let pill_bg: InheritableColor = LinearRgba::with_components(
            bg_lin.0 * 0.92 + fg_lin.0 * 0.08,
            bg_lin.1 * 0.92 + fg_lin.1 * 0.08,
            bg_lin.2 * 0.92 + fg_lin.2 * 0.08,
            bg_lin.3,
        )
        .into();

        // Active green status dot (~#5fa078)
        let active_green = LinearRgba::with_components(0.373, 0.627, 0.471, 1.0);

        // Dead/exited red dot (~#a05f5f)
        let dead_red = LinearRgba::with_components(0.627, 0.373, 0.373, 1.0);

        // Label dim: 40% fg + 60% bg (dimmer than dim_text which is 60/40)
        let label_dim: InheritableColor = LinearRgba::with_components(
            fg_lin.0 * 0.4 + bg_lin.0 * 0.6,
            fg_lin.1 * 0.4 + bg_lin.1 * 0.6,
            fg_lin.2 * 0.4 + bg_lin.2 * 0.6,
            fg_lin.3,
        )
        .into();

        let mut children: Vec<Element> = vec![];

        // --- A. Header with "FORGE" branding and window buttons ---
        let mut header_children: Vec<Element> = vec![];

        // "FORGE" text
        let forge_text = Element::new(&font, ElementContent::Text(" FORGE".to_string()))
            .display(DisplayType::Inline)
            .colors(ElementColors {
                border: BorderColor::default(),
                bg: InheritableColor::Inherited,
                text: bar_colors.text.clone(),
            });
        header_children.push(forge_text);

        // Window buttons (only if INTEGRATED_BUTTONS enabled)
        if self
            .config
            .window_decorations
            .contains(WindowDecorations::INTEGRATED_BUTTONS)
        {
            let is_maximized = self.window_state.contains(WindowState::MAXIMIZED);

            let buttons = [
                IntegratedTitleButton::Hide,
                IntegratedTitleButton::Maximize,
                IntegratedTitleButton::Close,
            ];

            let mut btn_children: Vec<Element> = vec![];
            for &button in &buttons {
                let btn = window_button_element(
                    button,
                    is_maximized,
                    &font,
                    &metrics,
                    &self.config,
                );
                // Override item_type from TabBar(WindowButton) to SidePanelWindowButton
                let btn = btn.item_type(UIItemType::SidePanelWindowButton(button));
                btn_children.push(btn);
            }

            let btn_container = Element::new(&font, ElementContent::Children(btn_children))
                .display(DisplayType::Inline)
                .float(Float::Right);
            header_children.push(btn_container);
        }

        let header = Element::new(&font, ElementContent::Children(header_children))
            .display(DisplayType::Block)
            .item_type(UIItemType::SidePanelHeader)
            .padding(BoxDimension {
                left: Dimension::Cells(0.5),
                right: Dimension::Cells(0.0),
                top: Dimension::Cells(0.3),
                bottom: Dimension::Cells(0.3),
            })
            .border(BoxDimension {
                left: Dimension::Pixels(0.0),
                right: Dimension::Pixels(0.0),
                top: Dimension::Pixels(0.0),
                bottom: Dimension::Pixels(1.0),
            })
            .colors(ElementColors {
                border: BorderColor::new(separator),
                bg: InheritableColor::Inherited,
                text: bar_colors.text.clone(),
            });
        children.push(header);

        // --- B. "SESSIONS" label ---
        let sessions_label =
            Element::new(&font, ElementContent::Text("  SESSIONS".to_string()))
                .display(DisplayType::Block)
                .padding(BoxDimension {
                    left: Dimension::Cells(0.0),
                    right: Dimension::Cells(0.0),
                    top: Dimension::Cells(0.5),
                    bottom: Dimension::Cells(0.3),
                })
                .colors(ElementColors {
                    border: BorderColor::default(),
                    bg: InheritableColor::Inherited,
                    text: label_dim.clone(),
                });
        children.push(sessions_label);

        // --- Collect tab info from mux (drop guards before building elements) ---
        struct TabInfo {
            idx: usize,
            is_active: bool,
            title: String,
            model: Option<String>,
            is_dead: bool,
        }
        let tab_infos: Vec<TabInfo> = {
            let mux = Mux::get();
            let mut infos = vec![];
            if let Some(window) = mux.get_window(self.mux_window_id) {
                let active_idx = window.get_active_idx();
                for (idx, tab) in window.iter().enumerate() {
                    let is_active = idx == active_idx;
                    let title = tab.get_title();
                    let is_dead = tab.is_dead();
                    let pane = tab.get_active_pane();
                    let session_info =
                        pane.as_ref().and_then(|p| p.claude_session_info());
                    let model = session_info.and_then(|info| info.model);
                    infos.push(TabInfo {
                        idx,
                        is_active,
                        title,
                        model,
                        is_dead,
                    });
                }
            }
            infos
        };

        // --- C. Tab rows with multi-child structure ---
        for tab_info in &tab_infos {
            // Display name computation (unchanged)
            let display_name = {
                let title_lower = tab_info.title.to_lowercase();
                let is_shell_or_empty = tab_info.title.is_empty()
                    || title_lower == "bash"
                    || title_lower == "zsh"
                    || title_lower == "fish"
                    || title_lower == "sh"
                    || title_lower.starts_with("bash ")
                    || title_lower.starts_with("zsh ")
                    || title_lower.starts_with("fish ");

                let base_name = if is_shell_or_empty {
                    format!("claude-{}", tab_info.idx + 1)
                } else {
                    tab_info.title.clone()
                };

                // Append " (exited)" if the tab/pane is dead or title contains "exited"
                let name_with_status = if tab_info.is_dead
                    || title_lower.contains("exited")
                {
                    if !base_name.contains("(exited)") {
                        format!("{} (exited)", base_name)
                    } else {
                        base_name
                    }
                } else {
                    base_name
                };

                // Truncate to fit the panel
                let max_chars = 18;
                if name_with_status.len() > max_chars {
                    format!("{}...", &name_with_status[..max_chars - 3])
                } else {
                    name_with_status
                }
            };

            // Model badge (abbreviated) — unchanged
            let model_badge = tab_info
                .model
                .as_deref()
                .map(|m| {
                    if m.contains("opus") {
                        "opus"
                    } else if m.contains("sonnet") {
                        "snnt"
                    } else if m.contains("haiku") {
                        "hku"
                    } else {
                        m.split('-').next().unwrap_or(m)
                    }
                })
                .unwrap_or("");

            let mut row_children: Vec<Element> = vec![];

            // Status dot (colored)
            let dot_color: InheritableColor = if tab_info.is_dead {
                dead_red.into()
            } else if tab_info.is_active {
                active_green.into()
            } else {
                dim_text.clone()
            };
            let dot_char = if tab_info.is_active || tab_info.is_dead {
                "\u{25cf}"
            } else {
                "\u{25cb}"
            };
            let dot_elem =
                Element::new(&font, ElementContent::Text(format!(" {}", dot_char)))
                    .display(DisplayType::Inline)
                    .colors(ElementColors {
                        border: BorderColor::default(),
                        bg: InheritableColor::Inherited,
                        text: dot_color,
                    });
            row_children.push(dot_elem);

            // Session name
            let name_text = if tab_info.is_active {
                bar_colors.text.clone()
            } else {
                dim_text.clone()
            };
            let name_elem = Element::new(
                &font,
                ElementContent::Text(format!(" {}", display_name)),
            )
            .display(DisplayType::Inline)
            .colors(ElementColors {
                border: BorderColor::default(),
                bg: InheritableColor::Inherited,
                text: name_text,
            });
            row_children.push(name_elem);

            // Model badge pill (floated right, rounded corners)
            if !model_badge.is_empty() {
                let corner_size = Dimension::Pixels(4.0);
                let badge = Element::new(
                    &font,
                    ElementContent::Text(model_badge.to_string()),
                )
                .display(DisplayType::Inline)
                .float(Float::Right)
                .padding(BoxDimension {
                    left: Dimension::Cells(0.3),
                    right: Dimension::Cells(0.3),
                    top: Dimension::Pixels(1.0),
                    bottom: Dimension::Pixels(1.0),
                })
                .border(BoxDimension::new(Dimension::Pixels(1.0)))
                .border_corners(Some(Corners {
                    top_left: SizedPoly {
                        width: corner_size,
                        height: corner_size,
                        poly: TOP_LEFT_ROUNDED_CORNER,
                    },
                    top_right: SizedPoly {
                        width: corner_size,
                        height: corner_size,
                        poly: TOP_RIGHT_ROUNDED_CORNER,
                    },
                    bottom_left: SizedPoly {
                        width: corner_size,
                        height: corner_size,
                        poly: BOTTOM_LEFT_ROUNDED_CORNER,
                    },
                    bottom_right: SizedPoly {
                        width: corner_size,
                        height: corner_size,
                        poly: BOTTOM_RIGHT_ROUNDED_CORNER,
                    },
                }))
                .colors(ElementColors {
                    border: BorderColor::new(separator),
                    bg: pill_bg.clone(),
                    text: dim_text.clone(),
                });
                row_children.push(badge);
            }

            // Active tab: 3px amber left border, elevated bg
            // Inactive: no visible left border
            let left_border_color = if tab_info.is_active {
                accent_amber
            } else {
                LinearRgba::TRANSPARENT
            };

            let row_bg = if tab_info.is_active {
                elevated_bg.clone()
            } else {
                InheritableColor::Inherited
            };

            let row = Element::new(&font, ElementContent::Children(row_children))
                .display(DisplayType::Block)
                .item_type(UIItemType::SidePanelTab(tab_info.idx))
                .padding(BoxDimension {
                    left: Dimension::Cells(0.0),
                    right: Dimension::Cells(0.3),
                    top: Dimension::Cells(0.15),
                    bottom: Dimension::Cells(0.15),
                })
                .border(BoxDimension {
                    left: Dimension::Pixels(3.0),
                    right: Dimension::Pixels(0.0),
                    top: Dimension::Pixels(0.0),
                    bottom: Dimension::Pixels(0.0),
                })
                .colors(ElementColors {
                    border: BorderColor::new(left_border_color),
                    bg: row_bg,
                    text: if tab_info.is_active {
                        bar_colors.text.clone()
                    } else {
                        dim_text.clone()
                    },
                })
                .hover_colors(Some(ElementColors {
                    border: BorderColor::new(accent_amber.mul_alpha(0.5)),
                    bg: elevated_bg.clone(),
                    text: bar_colors.text.clone(),
                }));

            children.push(row);
        }

        // --- D. Separator line ---
        let sep = Element::new(&font, ElementContent::Text(String::new()))
            .display(DisplayType::Block)
            .border(BoxDimension {
                left: Dimension::Pixels(0.0),
                right: Dimension::Pixels(0.0),
                top: Dimension::Pixels(0.0),
                bottom: Dimension::Pixels(1.0),
            })
            .colors(ElementColors {
                border: BorderColor::new(separator),
                bg: InheritableColor::Inherited,
                text: InheritableColor::Inherited,
            });
        children.push(sep);

        // --- E. "+ New Session" button with rounded corners ---
        let corner_size = Dimension::Pixels(6.0);
        let new_session = Element::new(
            &font,
            ElementContent::Text("  + New Session".to_string()),
        )
        .display(DisplayType::Block)
        .item_type(UIItemType::SidePanelNewButton)
        .padding(BoxDimension {
            left: Dimension::Cells(0.5),
            right: Dimension::Cells(0.5),
            top: Dimension::Cells(0.3),
            bottom: Dimension::Cells(0.3),
        })
        .margin(BoxDimension {
            left: Dimension::Pixels(8.0),
            right: Dimension::Pixels(8.0),
            top: Dimension::Cells(0.3),
            bottom: Dimension::Cells(0.0),
        })
        .border(BoxDimension::new(Dimension::Pixels(1.0)))
        .border_corners(Some(Corners {
            top_left: SizedPoly {
                width: corner_size,
                height: corner_size,
                poly: TOP_LEFT_ROUNDED_CORNER,
            },
            top_right: SizedPoly {
                width: corner_size,
                height: corner_size,
                poly: TOP_RIGHT_ROUNDED_CORNER,
            },
            bottom_left: SizedPoly {
                width: corner_size,
                height: corner_size,
                poly: BOTTOM_LEFT_ROUNDED_CORNER,
            },
            bottom_right: SizedPoly {
                width: corner_size,
                height: corner_size,
                poly: BOTTOM_RIGHT_ROUNDED_CORNER,
            },
        }))
        .colors(ElementColors {
            border: BorderColor::new(separator),
            bg: pill_bg.clone(),
            text: dim_text.clone(),
        })
        .hover_colors(Some(ElementColors {
            border: BorderColor::new(accent_amber.mul_alpha(0.5)),
            bg: elevated_bg.clone(),
            text: bar_colors.text.clone(),
        }));
        children.push(new_session);

        // --- F. Panel container with right border ---
        let panel_element = Element::new(&font, ElementContent::Children(children))
            .display(DisplayType::Block)
            .min_width(Some(Dimension::Pixels(self.side_panel_width)))
            .min_height(Some(Dimension::Pixels(
                self.dimensions.pixel_height as f32,
            )))
            .border(BoxDimension {
                left: Dimension::Pixels(0.0),
                right: Dimension::Pixels(1.0),
                top: Dimension::Pixels(0.0),
                bottom: Dimension::Pixels(0.0),
            })
            .colors(ElementColors {
                border: BorderColor::new(separator),
                bg: bar_colors.bg.clone(),
                text: bar_colors.text.clone(),
            });

        let border = self.get_os_border();

        let mut computed = self.compute_element(
            &LayoutContext {
                height: DimensionContext {
                    dpi: self.dimensions.dpi as f32,
                    pixel_max: self.dimensions.pixel_height as f32,
                    pixel_cell: metrics.cell_size.height as f32,
                },
                width: DimensionContext {
                    dpi: self.dimensions.dpi as f32,
                    pixel_max: self.side_panel_width,
                    pixel_cell: metrics.cell_size.width as f32,
                },
                bounds: euclid::rect(
                    0.,
                    0.,
                    self.side_panel_width,
                    self.dimensions.pixel_height as f32
                        - (border.top + border.bottom).get() as f32,
                ),
                metrics: &metrics,
                gl_state: self.render_state.as_ref().unwrap(),
                zindex: 10,
            },
            &panel_element,
        )?;

        computed.translate(euclid::vec2(
            border.left.get() as f32,
            border.top.get() as f32,
        ));

        Ok(computed)
    }

    pub fn paint_side_panel(&mut self) -> anyhow::Result<Vec<UIItem>> {
        if self.fancy_side_panel.is_none() {
            let palette = self.palette().clone();
            let computed = self.build_side_panel(&palette)?;
            self.fancy_side_panel = Some(computed);
        }

        let computed = self.fancy_side_panel.as_ref().unwrap();
        let mut ui_items = computed.ui_items();

        let gl_state = self.render_state.as_ref().unwrap();
        self.render_element(computed, gl_state, None)?;

        // Add a drag handle UIItem along the right edge of the side panel
        let border = self.get_os_border();
        let drag_handle_width = 4_usize;
        let panel_right = border.left.get() as usize + self.side_panel_width as usize;
        ui_items.push(UIItem {
            x: panel_right.saturating_sub(drag_handle_width),
            y: border.top.get() as usize,
            width: drag_handle_width * 2,
            height: self.dimensions.pixel_height,
            item_type: UIItemType::SidePanelDragHandle,
        });

        Ok(ui_items)
    }
}
