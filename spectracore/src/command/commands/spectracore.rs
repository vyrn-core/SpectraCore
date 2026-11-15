use spectracore_data::packet::CURRENT_MC_PROTOCOL;
use spectracore_util::text::click::ClickEvent;
use spectracore_util::text::hover::HoverEvent;
use spectracore_util::text::{TextComponent, color::NamedColor};
use spectracore_util::translation::get_translation_text;
use spectracore_world::CURRENT_MC_VERSION;
use std::borrow::Cow;

use crate::command::CommandResult;
use crate::command::{CommandExecutor, CommandSender, args::ConsumedArgs, tree::CommandTree};

const NAMES: [&str; 2] = ["spectracore", "version"];

const DESCRIPTION: &str = "Display information about SpectraCore.";

struct Executor;

const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

#[allow(clippy::too_many_lines)]
impl CommandExecutor for Executor {
    fn execute<'a>(
        &'a self,
        sender: &'a CommandSender,
        _server: &'a crate::server::Server,
        _args: &'a ConsumedArgs<'a>,
    ) -> CommandResult<'a> {
        Box::pin(async move {
            let locale = sender.get_locale().await;
            sender
                .send_message(
                    TextComponent::custom(
                        "spectracore",
                        "commands.spectracore.version",
                        locale,
                        vec![TextComponent::text(CARGO_PKG_VERSION)],
                    )
                    .hover_event(HoverEvent::show_text(TextComponent::custom(
                        "spectracore",
                        "commands.spectracore.version.hover",
                        locale,
                        vec![],
                    )))
                    .click_event(ClickEvent::CopyToClipboard {
                        value: Cow::from(
                            get_translation_text(
                                "spectracore:commands.spectracore.version",
                                locale,
                                vec![TextComponent::text(CARGO_PKG_VERSION).0],
                            )
                            .replace('\n', ""),
                        ),
                    })
                    .color_named(NamedColor::Green)
                    .add_child(
                        TextComponent::custom(
                            "spectracore",
                            "commands.spectracore.description",
                            locale,
                            vec![],
                        )
                        .click_event(ClickEvent::CopyToClipboard {
                            value: Cow::from(
                                get_translation_text(
                                    "spectracore:commands.spectracore.description",
                                    locale,
                                    vec![],
                                )
                                .replace('\n', ""),
                            ),
                        })
                        .hover_event(HoverEvent::show_text(TextComponent::custom(
                            "spectracore",
                            "commands.spectracore.description.hover",
                            locale,
                            vec![],
                        )))
                        .color_named(NamedColor::White),
                    )
                    .add_child(
                        TextComponent::custom(
                            "spectracore",
                            "commands.spectracore.minecraft_version",
                            locale,
                            vec![
                                TextComponent::text(CURRENT_MC_VERSION),
                                TextComponent::text(format!("{CURRENT_MC_PROTOCOL}")),
                            ],
                        )
                        .click_event(ClickEvent::CopyToClipboard {
                            value: Cow::from(
                                get_translation_text(
                                    "spectracore:commands.spectracore.minecraft_version",
                                    locale,
                                    vec![
                                        TextComponent::text(CURRENT_MC_VERSION).0,
                                        TextComponent::text(format!("{CURRENT_MC_PROTOCOL}")).0,
                                    ],
                                )
                                .replace('\n', ""),
                            ),
                        })
                        .hover_event(HoverEvent::show_text(TextComponent::custom(
                            "spectracore",
                            "commands.spectracore.minecraft_version.hover",
                            locale,
                            vec![],
                        )))
                        .color_named(NamedColor::Gold),
                    )
                    // https://spectracoremc.org/
                    .add_child(
                        TextComponent::custom("spectracore", "commands.spectracore.github", locale, vec![])
                            .click_event(ClickEvent::OpenUrl {
                                url: Cow::from("https://github.com/SpectraCore-MC/SpectraCore"),
                            })
                            .hover_event(HoverEvent::show_text(TextComponent::custom(
                                "spectracore",
                                "commands.spectracore.github.hover",
                                locale,
                                vec![],
                            )))
                            .color_named(NamedColor::Blue)
                            .bold()
                            .underlined(),
                    )
                    // Added docs. and a space for spacing
                    .add_child(TextComponent::text("  "))
                    .add_child(
                        TextComponent::custom(
                            "spectracore",
                            "commands.spectracore.website",
                            locale,
                            vec![],
                        )
                        .click_event(ClickEvent::OpenUrl {
                            url: Cow::from("https://spectracoremc.org/"),
                        })
                        .hover_event(HoverEvent::show_text(TextComponent::custom(
                            "spectracore",
                            "commands.spectracore.website.hover",
                            locale,
                            vec![],
                        )))
                        .color_named(NamedColor::Blue)
                        .bold()
                        .underlined(),
                    ),
                )
                .await;
            Ok(())
        })
    }
}

pub fn init_command_tree() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION).execute(Executor)
}
