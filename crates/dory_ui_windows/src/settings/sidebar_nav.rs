use super::{SettingsCoordinator, SettingsFocus, SettingsSectionId};
use dory_components::components::tree_nav::{TreeNav, TreeNavNode};
use dory_components::icons::AppIcon;
use gpui::SharedString;
use std::collections::HashSet;

impl SettingsCoordinator {
    #[allow(clippy::result_large_err)]
    pub(super) fn build_sidebar_tree() -> TreeNav {
        // Groups match the Figma design: NETWORK / CONNECTION / GENERAL
        let nodes = vec![
            TreeNavNode::group(
                "general-group",
                dory_i18n::t!("settings.nav.general_group"),
                Some(AppIcon::Settings),
                vec![
                    TreeNavNode::leaf(
                        "general",
                        dory_i18n::t!("settings.nav.general"),
                        Some(AppIcon::Settings),
                    ),
                    TreeNavNode::leaf(
                        "keybindings",
                        dory_i18n::t!("settings.nav.keybindings"),
                        Some(AppIcon::Keyboard),
                    ),
                    TreeNavNode::leaf(
                        "audit",
                        dory_i18n::t!("settings.nav.audit"),
                        Some(AppIcon::History),
                    ),
                    TreeNavNode::leaf(
                        "about",
                        dory_i18n::t!("settings.nav.about"),
                        Some(AppIcon::Info),
                    ),
                ],
            ),
            TreeNavNode::group(
                "network",
                dory_i18n::t!("settings.nav.network"),
                Some(AppIcon::Server),
                vec![
                    TreeNavNode::leaf(
                        "ssh-tunnels",
                        dory_i18n::t!("settings.nav.ssh_tunnels"),
                        Some(AppIcon::FingerprintPattern),
                    ),
                    TreeNavNode::leaf(
                        "proxies",
                        dory_i18n::t!("settings.nav.proxies"),
                        Some(AppIcon::Server),
                    ),
                    TreeNavNode::leaf(
                        "auth-profiles",
                        dory_i18n::t!("settings.nav.auth_profiles"),
                        Some(AppIcon::KeyRound),
                    ),
                ],
            ),
            TreeNavNode::group(
                "connection",
                dory_i18n::t!("settings.nav.connection"),
                Some(AppIcon::Link2),
                vec![
                    TreeNavNode::leaf(
                        "hooks",
                        dory_i18n::t!("settings.nav.hooks"),
                        Some(AppIcon::SquareTerminal),
                    ),
                    TreeNavNode::leaf(
                        "drivers",
                        dory_i18n::t!("settings.nav.drivers"),
                        Some(AppIcon::Database),
                    ),
                    TreeNavNode::leaf(
                        "services",
                        dory_i18n::t!("settings.nav.services"),
                        Some(AppIcon::Plug),
                    ),
                ],
            ),
            #[cfg(feature = "mcp")]
            TreeNavNode::group(
                "mcp-governance",
                dory_i18n::t!("settings.nav.mcp_governance"),
                Some(AppIcon::Bot),
                vec![
                    TreeNavNode::leaf(
                        "mcp-clients",
                        dory_i18n::t!("settings.nav.mcp_clients"),
                        Some(AppIcon::Plug),
                    ),
                    TreeNavNode::leaf(
                        "mcp-roles",
                        dory_i18n::t!("settings.nav.mcp_roles"),
                        Some(AppIcon::KeyRound),
                    ),
                    TreeNavNode::leaf(
                        "mcp-policies",
                        dory_i18n::t!("settings.nav.mcp_policies"),
                        Some(AppIcon::ScrollText),
                    ),
                ],
            ),
        ];

        let mut expanded = HashSet::new();
        #[cfg(feature = "mcp")]
        expanded.insert(SharedString::from("mcp-governance"));
        expanded.insert(SharedString::from("network"));
        expanded.insert(SharedString::from("connection"));
        expanded.insert(SharedString::from("general-group"));

        TreeNav::new(nodes, expanded)
    }

    pub(super) fn section_for_tree_id(id: &str) -> Option<SettingsSectionId> {
        match id {
            "general" => Some(SettingsSectionId::General),
            "audit" => Some(SettingsSectionId::Audit),
            #[cfg(feature = "mcp")]
            "mcp-clients" => Some(SettingsSectionId::McpClients),
            #[cfg(feature = "mcp")]
            "mcp-roles" => Some(SettingsSectionId::McpRoles),
            #[cfg(feature = "mcp")]
            "mcp-policies" => Some(SettingsSectionId::McpPolicies),
            "keybindings" => Some(SettingsSectionId::Keybindings),
            "proxies" => Some(SettingsSectionId::Proxies),
            "ssh-tunnels" => Some(SettingsSectionId::SshTunnels),
            "auth-profiles" => Some(SettingsSectionId::AuthProfiles),
            "services" => Some(SettingsSectionId::Services),
            "hooks" => Some(SettingsSectionId::Hooks),
            "drivers" => Some(SettingsSectionId::Drivers),
            "about" => Some(SettingsSectionId::About),
            _ => None,
        }
    }

    pub(super) fn tree_id_for_section(section: SettingsSectionId) -> &'static str {
        match section {
            SettingsSectionId::General => "general",
            SettingsSectionId::Audit => "audit",
            #[cfg(feature = "mcp")]
            SettingsSectionId::McpClients => "mcp-clients",
            #[cfg(feature = "mcp")]
            SettingsSectionId::McpRoles => "mcp-roles",
            #[cfg(feature = "mcp")]
            SettingsSectionId::McpPolicies => "mcp-policies",
            SettingsSectionId::Keybindings => "keybindings",
            SettingsSectionId::Proxies => "proxies",
            SettingsSectionId::SshTunnels => "ssh-tunnels",
            SettingsSectionId::AuthProfiles => "auth-profiles",
            SettingsSectionId::Services => "services",
            SettingsSectionId::Hooks => "hooks",
            SettingsSectionId::Drivers => "drivers",
            SettingsSectionId::About => "about",
        }
    }

    #[allow(dead_code)]
    pub(super) fn focus_sidebar(&mut self) {
        self.focus_area = SettingsFocus::Sidebar;
        self.sidebar_tree
            .select_by_id(Self::tree_id_for_section(self.active_section));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_for_tree_id_known_ids() {
        assert_eq!(
            SettingsCoordinator::section_for_tree_id("general"),
            Some(SettingsSectionId::General)
        );
        assert_eq!(
            SettingsCoordinator::section_for_tree_id("audit"),
            Some(SettingsSectionId::Audit)
        );
        assert_eq!(
            SettingsCoordinator::section_for_tree_id("proxies"),
            Some(SettingsSectionId::Proxies)
        );
        #[cfg(feature = "mcp")]
        {
            assert_eq!(
                SettingsCoordinator::section_for_tree_id("mcp-clients"),
                Some(SettingsSectionId::McpClients)
            );
            assert_eq!(
                SettingsCoordinator::section_for_tree_id("mcp-policies"),
                Some(SettingsSectionId::McpPolicies)
            );
        }
    }

    #[test]
    fn section_for_tree_id_unknown_returns_none() {
        assert_eq!(
            SettingsCoordinator::section_for_tree_id("nonexistent"),
            None
        );
        assert_eq!(SettingsCoordinator::section_for_tree_id("mcp"), None);
    }

    #[test]
    fn tree_id_roundtrip_all_sections() {
        let mut sections = vec![
            SettingsSectionId::General,
            SettingsSectionId::Audit,
            SettingsSectionId::Keybindings,
            SettingsSectionId::Proxies,
            SettingsSectionId::SshTunnels,
            SettingsSectionId::AuthProfiles,
            SettingsSectionId::Services,
            SettingsSectionId::Hooks,
            SettingsSectionId::Drivers,
            SettingsSectionId::About,
        ];

        #[cfg(feature = "mcp")]
        {
            sections.extend([
                SettingsSectionId::McpClients,
                SettingsSectionId::McpRoles,
                SettingsSectionId::McpPolicies,
            ]);
        }

        for section in sections {
            let id = SettingsCoordinator::tree_id_for_section(section);
            assert_eq!(SettingsCoordinator::section_for_tree_id(id), Some(section));
        }
    }

    #[test]
    fn services_tree_label_uses_neutral_rpc_wording() {
        let tree = SettingsCoordinator::build_sidebar_tree();
        let services_row = tree
            .rows()
            .iter()
            .find(|row| row.id.as_ref() == "services")
            .expect("services row");

        assert_eq!(
            services_row.label.as_ref(),
            dory_i18n::t!("settings.nav.services")
        );
    }

    const NAV_CATALOG_KEYS: &[&str] = &[
        "settings.nav.general_group",
        "settings.nav.general",
        "settings.nav.keybindings",
        "settings.nav.audit",
        "settings.nav.about",
        "settings.nav.network",
        "settings.nav.ssh_tunnels",
        "settings.nav.proxies",
        "settings.nav.auth_profiles",
        "settings.nav.connection",
        "settings.nav.hooks",
        "settings.nav.drivers",
        "settings.nav.services",
        "settings.nav.mcp_governance",
        "settings.nav.mcp_clients",
        "settings.nav.mcp_roles",
        "settings.nav.mcp_policies",
    ];

    #[test]
    fn settings_nav_keys_resolve_in_both_locales() {
        for locale in ["en", "es"] {
            for key in NAV_CATALOG_KEYS {
                let value = dory_i18n::t!(key, locale = locale);

                assert!(
                    !value.is_empty(),
                    "key {key} resolved empty for locale {locale}"
                );
                assert_ne!(value, *key, "key {key} did not resolve for locale {locale}");
                assert_ne!(
                    value,
                    format!("{locale}.{key}"),
                    "key {key} fell back to the raw locale-qualified form for locale {locale}"
                );
            }
        }
    }

    #[test]
    fn settings_nav_keybindings_differs_between_locales() {
        let english = dory_i18n::t!("settings.nav.keybindings", locale = "en");
        let spanish = dory_i18n::t!("settings.nav.keybindings", locale = "es");

        assert_eq!(english, "Keybindings");
        assert_eq!(spanish, "Atajos de teclado");
        assert_ne!(english, spanish);
    }
}
