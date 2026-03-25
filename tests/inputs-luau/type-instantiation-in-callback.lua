-- Test for issue #1088: turbofish/TypeInstantiation inside nested function call
Charm.computed(function()
    return createPluginSettingsStore<<AuthStore>>("PresenceAuthStore", defaultValue, validate)
end)
