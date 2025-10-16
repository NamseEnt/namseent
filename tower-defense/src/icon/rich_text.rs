use crate::icon::{Icon, IconAttribute, IconAttributePosition, IconKind, IconSize};
use namui::*;

#[derive(Debug, Clone, State)]
pub enum IconParseError {
    Format,
    Kind,
    Size,
    Dimensions,
    Opacity,
    Attribute,
}

impl Icon {
    pub fn as_tag(&self) -> String {
        let Self {
            kind,
            size,
            attributes,
            wh,
            opacity,
        } = self;

        let kind_part = kind.asset_id();
        let size_part = size.px().as_f32().to_string();
        let width_part = wh.width.as_f32().to_string();
        let height_part = wh.height.as_f32().to_string();
        let opacity_part = opacity.to_string();

        let attr_part = if attributes.is_empty() {
            String::new()
        } else {
            let attr_strings: Vec<String> = attributes
                .iter()
                .map(|attr| format!("{}_{}", attr.icon_kind.asset_id(), attr.position.as_str()))
                .collect();
            attr_strings.join(",")
        };

        if attr_part.is_empty() {
            format!("icon<{kind_part}:{size_part}:{width_part}:{height_part}:{opacity_part}>")
        } else {
            format!(
                "icon<{kind_part}:{size_part}:{width_part}:{height_part}:{opacity_part}:{attr_part}>"
            )
        }
    }

    pub fn from_tag(tag: &str) -> Result<Self, IconParseError> {
        // Check if tag starts with "icon<" and ends with ">"
        if !tag.starts_with("icon<") || !tag.ends_with(">") {
            return Err(IconParseError::Format);
        }

        // Extract content between icon< and >
        let content = &tag[5..tag.len() - 1];

        // Split by : to get main parts
        let parts: Vec<&str> = content.split(':').collect();

        // Minimum required parts: kind, size, width, height, opacity
        if parts.len() < 5 {
            return Err(IconParseError::Format);
        }

        // Parse kind
        let kind = IconKind::from_asset_id(parts[0]).ok_or(IconParseError::Kind)?;

        // Parse size
        let size_value = parts[1].parse::<f32>().map_err(|_| IconParseError::Size)?;
        let size = IconSize::Custom {
            size: px(size_value),
        };

        // Parse width and height
        let width = parts[2]
            .parse::<f32>()
            .map_err(|_| IconParseError::Dimensions)?;
        let height = parts[3]
            .parse::<f32>()
            .map_err(|_| IconParseError::Dimensions)?;
        let wh = Wh {
            width: px(width),
            height: px(height),
        };

        // Parse opacity
        let opacity = parts[4]
            .parse::<f32>()
            .map_err(|_| IconParseError::Opacity)?;

        // Parse attributes (if exists)
        let mut attributes = Vec::new();
        if parts.len() > 5 {
            let attr_content = parts[5];
            if !attr_content.is_empty() {
                let attr_items: Vec<&str> = attr_content.split(',').collect();
                for attr_item in attr_items {
                    // Find the position indicators at the end after the last underscore
                    let position_parts = [
                        "_top_left",
                        "_top_right",
                        "_bottom_left",
                        "_bottom_right",
                        "_center",
                    ];
                    let mut found_position = None;
                    let mut attr_kind_str = attr_item;

                    for &pos_suffix in &position_parts {
                        if attr_item.ends_with(pos_suffix) {
                            found_position = Some(&pos_suffix[1..]); // Remove the leading underscore
                            attr_kind_str = attr_item.strip_suffix(pos_suffix).unwrap();
                            break;
                        }
                    }

                    if let Some(pos_str) = found_position {
                        let attr_kind = IconKind::from_asset_id(attr_kind_str)
                            .ok_or(IconParseError::Attribute)?;
                        let attr_position = IconAttributePosition::from_str(pos_str)
                            .ok_or(IconParseError::Attribute)?;

                        attributes.push(IconAttribute {
                            icon_kind: attr_kind,
                            position: attr_position,
                        });
                    } else {
                        return Err(IconParseError::Attribute);
                    }
                }
            }
        }

        Ok(Icon {
            kind,
            size,
            attributes,
            wh,
            opacity,
        })
    }

    /// Regular expression pattern for matching icon tags
    pub fn tag_regex_pattern() -> &'static str {
        r"icon<[^:>]+:[0-9]+(?:\.[0-9]+)?:[0-9]+(?:\.[0-9]+)?:[0-9]+(?:\.[0-9]+)?:[0-9]+(?:\.[0-9]+)?(?::[^>]*(?:_(?:top_left|top_right|bottom_left|bottom_right|center)(?:,[^>]*_(?:top_left|top_right|bottom_left|bottom_right|center))*)?)?>"
    }

    /// Check if a string matches the icon tag format using regex
    pub fn is_valid_tag_format(tag: &str) -> bool {
        // Basic structure check
        if !tag.starts_with("icon<") || !tag.ends_with(">") || tag.len() <= 6 {
            return false;
        }

        // Extract content and split by :
        let content = &tag[5..tag.len() - 1];
        let parts: Vec<&str> = content.split(':').collect();

        // Must have at least 5 parts (kind, size, width, height, opacity)
        if parts.len() < 5 {
            return false;
        }

        // Check that numeric parts can be parsed
        for part in parts.iter().take(5).skip(1) {
            if part.parse::<f32>().is_err() {
                return false;
            }
        }

        // If there are attributes, check their format
        if parts.len() > 5 {
            let attr_content = parts[5];
            if !attr_content.is_empty() {
                let attr_items: Vec<&str> = attr_content.split(',').collect();
                let position_suffixes = [
                    "_top_left",
                    "_top_right",
                    "_bottom_left",
                    "_bottom_right",
                    "_center",
                ];

                for attr_item in attr_items {
                    let has_valid_position = position_suffixes
                        .iter()
                        .any(|&suffix| attr_item.ends_with(suffix));
                    if !has_valid_position {
                        return false;
                    }
                }
            }
        }

        true
    }

    /// Create regex handlers for icon pattern matching in rich text
    pub fn create_icon_regex_handlers() -> Vec<namui_prebuilt::rich_text::RegexHandler> {
        vec![
            namui_prebuilt::rich_text::RegexHandler::new(
                r"icon<[^>]+>",
                Box::new(move |matched_text| {
                    // Try to parse the icon tag
                    if let Ok(icon) = Icon::from_tag(matched_text) {
                        icon.to_rendering_tree()
                    } else {
                        // Fallback to error placeholder
                        Self::render_icon_error_fallback(matched_text)
                    }
                }),
            )
            .unwrap_or_else(|_| {
                println!("Failed to create regex handler for icon tag");
                // If regex compilation fails, create a dummy handler
                namui_prebuilt::rich_text::RegexHandler::new(
                    r"icon<[^>]+>",
                    Box::new(|_| RenderingTree::Empty),
                )
                .unwrap()
            }),
        ]
    }

    /// Render a fallback error icon when parsing fails
    pub fn render_icon_error_fallback(_matched_text: &str) -> RenderingTree {
        namui::rect(RectParam {
            rect: Rect::Xywh {
                x: 0.px(),
                y: 0.px(),
                width: 16.px(),
                height: 16.px(),
            },
            style: RectStyle {
                fill: Some(RectFill {
                    color: Color::from_u8(0, 0, 0, 32),
                }),
                stroke: None,
                ..Default::default()
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Suit;

    #[test]
    fn test_icon_serialization_deserialization() {
        let original_icon = Icon {
            kind: IconKind::Gold,
            size: IconSize::Custom { size: px(24.0) },
            attributes: vec![
                IconAttribute {
                    icon_kind: IconKind::AttackDamage,
                    position: IconAttributePosition::TopLeft,
                },
                IconAttribute {
                    icon_kind: IconKind::Shield,
                    position: IconAttributePosition::BottomRight,
                },
            ],
            wh: Wh {
                width: px(32.0),
                height: px(32.0),
            },
            opacity: 0.8,
        };

        let tag = original_icon.as_tag();
        let deserialized_icon = Icon::from_tag(&tag).unwrap();

        assert_eq!(original_icon.kind, deserialized_icon.kind);
        assert_eq!(original_icon.size.px(), deserialized_icon.size.px());
        assert_eq!(original_icon.wh, deserialized_icon.wh);
        assert_eq!(original_icon.opacity, deserialized_icon.opacity);
        assert_eq!(original_icon.attributes, deserialized_icon.attributes);
    }

    #[test]
    fn test_icon_with_suit_serialization() {
        let icon = Icon {
            kind: IconKind::Suit { suit: Suit::Hearts },
            size: IconSize::Custom { size: px(16.0) },
            attributes: vec![],
            wh: Wh {
                width: px(20.0),
                height: px(20.0),
            },
            opacity: 1.0,
        };

        let tag = icon.as_tag();
        let deserialized = Icon::from_tag(&tag).unwrap();

        assert_eq!(icon.kind, deserialized.kind);
    }

    #[test]
    fn test_invalid_tag_format() {
        assert!(matches!(
            Icon::from_tag("invalid"),
            Err(IconParseError::Format)
        ));
        assert!(matches!(
            Icon::from_tag("not_icon<gold:24:32:32:1.0>"),
            Err(IconParseError::Format)
        ));
        assert!(matches!(
            Icon::from_tag("icon<invalid_kind:24:32:32:1.0>"),
            Err(IconParseError::Kind)
        ));
        assert!(matches!(
            Icon::from_tag("icon<gold:invalid:32:32:1.0>"),
            Err(IconParseError::Size)
        ));
        assert!(matches!(
            Icon::from_tag("icon<gold:24:invalid:32:1.0>"),
            Err(IconParseError::Dimensions)
        ));
        assert!(matches!(
            Icon::from_tag("icon<gold:24:32:32:invalid>"),
            Err(IconParseError::Opacity)
        ));
        assert!(matches!(
            Icon::from_tag("icon<gold:24:32:32:1.0:invalid_attr>"),
            Err(IconParseError::Attribute)
        ));
    }

    #[test]
    fn test_tag_format_examples() {
        // Test simple icon without attributes
        let simple_icon = Icon {
            kind: IconKind::Gold,
            size: IconSize::Custom { size: px(24.0) },
            attributes: vec![],
            wh: Wh {
                width: px(32.0),
                height: px(32.0),
            },
            opacity: 1.0,
        };

        let tag = simple_icon.as_tag();
        println!("Simple icon tag: {tag}");
        assert_eq!(tag, "icon<gold:24:32:32:1>");

        // Test icon with attributes
        let complex_icon = Icon {
            kind: IconKind::AttackDamage,
            size: IconSize::Custom { size: px(16.0) },
            attributes: vec![
                IconAttribute {
                    icon_kind: IconKind::Shield,
                    position: IconAttributePosition::TopLeft,
                },
                IconAttribute {
                    icon_kind: IconKind::Gold,
                    position: IconAttributePosition::BottomRight,
                },
            ],
            wh: Wh {
                width: px(20.0),
                height: px(20.0),
            },
            opacity: 0.8,
        };

        let complex_tag = complex_icon.as_tag();
        println!("Complex icon tag: {complex_tag}");
        assert_eq!(
            complex_tag,
            "icon<attack_damage:16:20:20:0.8:shield_top_left,gold_bottom_right>"
        );

        // Test round-trip
        let restored = Icon::from_tag(&complex_tag).unwrap();
        assert_eq!(complex_icon.kind, restored.kind);
        assert_eq!(complex_icon.attributes, restored.attributes);
    }

    #[test]
    fn test_tag_format_validation() {
        // Valid tags
        assert!(Icon::is_valid_tag_format("icon<gold:24:32:32:1>"));
        assert!(Icon::is_valid_tag_format(
            "icon<attack_damage:16:20:20:0.8:shield_top_left>"
        ));
        assert!(Icon::is_valid_tag_format(
            "icon<suit_hearts:12:16:16:1.0:gold_center,shield_bottom_right>"
        ));

        // Invalid tags
        assert!(!Icon::is_valid_tag_format("not_icon<gold:24:32:32:1>"));
        assert!(!Icon::is_valid_tag_format("icon<gold:24:32:32:1"));
        assert!(!Icon::is_valid_tag_format("icon<gold:24:32:32>"));
        assert!(!Icon::is_valid_tag_format("icon<gold:invalid:32:32:1>"));
        assert!(!Icon::is_valid_tag_format("icon<gold:24:32:32:1:invalid>"));

        // Show regex pattern for reference
        println!("Icon tag regex pattern: {}", Icon::tag_regex_pattern());
    }

    #[test]
    fn test_create_icon_regex_handlers() {
        let handlers = Icon::create_icon_regex_handlers();
        assert_eq!(handlers.len(), 1);
    }

    #[test]
    fn test_render_icon_error_fallback() {
        let fallback = Icon::render_icon_error_fallback("invalid_tag");
        // Should return a RenderingTree (basic structure test)
        if fallback == RenderingTree::Empty {
            panic!("Expected non-empty rendering tree")
        }
    }
}
