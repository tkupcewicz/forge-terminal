use crate::termwindow::box_model::*;
use crate::termwindow::{UIItem, UIItemType};
use crate::utilsprites::RenderMetrics;
use config::{Dimension, DimensionContext, TabBarColors};
use mux::Mux;
use wezterm_term::color::ColorPalette;

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

        // Dimmer text color for non-highlighted elements
        let dim_text: InheritableColor = {
            let base = if self.focused.is_some() {
                self.config.window_frame.active_titlebar_fg
            } else {
                self.config.window_frame.inactive_titlebar_fg
            }
            .to_linear();
            // Mix with background to make it dimmer
            let bg = if self.focused.is_some() {
                self.config.window_frame.active_titlebar_bg
            } else {
                self.config.window_frame.inactive_titlebar_bg
            }
            .to_linear();
            window::color::LinearRgba::with_components(
                base.0 * 0.6 + bg.0 * 0.4,
                base.1 * 0.6 + bg.1 * 0.4,
                base.2 * 0.6 + bg.2 * 0.4,
                base.3,
            )
            .into()
        };

        let mut children: Vec<Element> = vec![];

        // "SESSIONS" header
        let header = Element::new(&font, ElementContent::Text("  SESSIONS".to_string()))
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
                text: dim_text.clone(),
            });
        children.push(header);

        // Collect tab info from mux (drop guards before building elements)
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

        // Build tab row elements from collected info
        for tab_info in &tab_infos {
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

            // Status dot
            let dot = if tab_info.is_active {
                "\u{25cf}"
            } else {
                "\u{25cb}"
            };

            // Model badge (abbreviated)
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

            // Format the row: " ● name          opus"
            // Pad the name to align the badge
            let name_width = 16;
            let padded_name = if display_name.len() >= name_width {
                display_name[..name_width].to_string()
            } else {
                format!(
                    "{}{:width$}",
                    display_name,
                    "",
                    width = name_width - display_name.len()
                )
            };
            let row_text = format!("  {} {} {}", dot, padded_name, model_badge);

            let active_highlight_bg: InheritableColor = if tab_info.is_active {
                let bg = if self.focused.is_some() {
                    self.config.window_frame.active_titlebar_bg
                } else {
                    self.config.window_frame.inactive_titlebar_bg
                }
                .to_linear();
                let fg = if self.focused.is_some() {
                    self.config.window_frame.active_titlebar_fg
                } else {
                    self.config.window_frame.inactive_titlebar_fg
                }
                .to_linear();
                // Slightly lighter/different from panel background
                window::color::LinearRgba::with_components(
                    bg.0 * 0.85 + fg.0 * 0.15,
                    bg.1 * 0.85 + fg.1 * 0.15,
                    bg.2 * 0.85 + fg.2 * 0.15,
                    bg.3,
                )
                .into()
            } else {
                InheritableColor::Inherited
            };

            let row_colors = ElementColors {
                border: BorderColor::default(),
                bg: active_highlight_bg,
                text: if tab_info.is_active {
                    bar_colors.text.clone()
                } else {
                    dim_text.clone()
                },
            };

            let hover_colors = Some(ElementColors {
                border: BorderColor::default(),
                bg: {
                    let bg = if self.focused.is_some() {
                        self.config.window_frame.active_titlebar_bg
                    } else {
                        self.config.window_frame.inactive_titlebar_bg
                    }
                    .to_linear();
                    let fg = if self.focused.is_some() {
                        self.config.window_frame.active_titlebar_fg
                    } else {
                        self.config.window_frame.inactive_titlebar_fg
                    }
                    .to_linear();
                    window::color::LinearRgba::with_components(
                        bg.0 * 0.8 + fg.0 * 0.2,
                        bg.1 * 0.8 + fg.1 * 0.2,
                        bg.2 * 0.8 + fg.2 * 0.2,
                        bg.3,
                    )
                    .into()
                },
                text: bar_colors.text.clone(),
            });

            let row = Element::new(&font, ElementContent::Text(row_text))
                .display(DisplayType::Block)
                .item_type(UIItemType::SidePanelTab(tab_info.idx))
                .padding(BoxDimension {
                    left: Dimension::Cells(0.0),
                    right: Dimension::Cells(0.0),
                    top: Dimension::Cells(0.15),
                    bottom: Dimension::Cells(0.15),
                })
                .colors(row_colors)
                .hover_colors(hover_colors);

            children.push(row);
        }

        // "+ New Session" button
        let new_session = Element::new(
            &font,
            ElementContent::Text("  + New Session".to_string()),
        )
        .display(DisplayType::Block)
        .item_type(UIItemType::SidePanelNewButton)
        .padding(BoxDimension {
            left: Dimension::Cells(0.0),
            right: Dimension::Cells(0.0),
            top: Dimension::Cells(0.4),
            bottom: Dimension::Cells(0.15),
        })
        .colors(ElementColors {
            border: BorderColor::default(),
            bg: InheritableColor::Inherited,
            text: dim_text,
        })
        .hover_colors(Some(ElementColors {
            border: BorderColor::default(),
            bg: {
                let bg = if self.focused.is_some() {
                    self.config.window_frame.active_titlebar_bg
                } else {
                    self.config.window_frame.inactive_titlebar_bg
                }
                .to_linear();
                let fg = if self.focused.is_some() {
                    self.config.window_frame.active_titlebar_fg
                } else {
                    self.config.window_frame.inactive_titlebar_fg
                }
                .to_linear();
                window::color::LinearRgba::with_components(
                    bg.0 * 0.8 + fg.0 * 0.2,
                    bg.1 * 0.8 + fg.1 * 0.2,
                    bg.2 * 0.8 + fg.2 * 0.2,
                    bg.3,
                )
                .into()
            },
            text: bar_colors.text.clone(),
        }));
        children.push(new_session);

        // Build the outer panel container
        let panel_element = Element::new(&font, ElementContent::Children(children))
            .display(DisplayType::Block)
            .min_width(Some(Dimension::Pixels(self.side_panel_width)))
            .min_height(Some(Dimension::Pixels(
                self.dimensions.pixel_height as f32,
            )))
            .colors(bar_colors);

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
