use dory_components::typography::AppFonts;

#[test]
fn app_fonts_define_shared_family_contract() {
    assert_eq!(AppFonts::BODY, AppFonts::SYSTEM);
    assert_eq!(AppFonts::HEADLINE, AppFonts::SYSTEM);
    assert_eq!(AppFonts::MONO, "monospace");
    assert_eq!(AppFonts::CODE, AppFonts::MONO);
    assert_eq!(AppFonts::SHORTCUT, AppFonts::MONO);
}
