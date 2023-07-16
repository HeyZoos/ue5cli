struct UnrealManagerBase;

impl UnrealManagerBase {
    fn valid_build_configurations() -> Vec<&'static str> {
        vec!["Debug", "DebugGame", "Development", "Shipping", "Test"]
    }

    fn set_engine_root_override() {}
}
