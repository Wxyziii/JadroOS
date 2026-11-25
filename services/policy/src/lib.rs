#![forbid(unsafe_code)]
//! Pure reference policy decisions only. Not an authorization or execution service.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab { General, ChatGpt, Claude, Codex, ClaudeCode, Local }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effort { Instant, Medium, High, ExtraHigh }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route { LocalSearch, OfficialChatGpt, NativeOpenAi, OfficialClaude, CodexAgent, ClaudeCodeCli, LocalModel, Unavailable }

/// The capability flag must eventually come from an authenticated capability registry.
/// No automatic fallback from Instant to native inference.
pub fn route(tab: Tab, effort: Effort, native_openai_available: bool) -> Route {
    match (tab, effort) {
        (Tab::General, _) => Route::LocalSearch,
        (Tab::ChatGpt, Effort::Instant) => Route::OfficialChatGpt,
        (Tab::ChatGpt, _) if native_openai_available => Route::NativeOpenAi,
        (Tab::ChatGpt, _) => Route::Unavailable,
        (Tab::Claude, _) => Route::OfficialClaude,
        (Tab::Codex, _) => Route::CodexAgent,
        (Tab::ClaudeCode, _) => Route::ClaudeCodeCli,
        (Tab::Local, _) => Route::LocalModel,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Risk { Read, Write, Administrative, Prohibited }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision { Deny, PermitScopedRead, RequireApproval }

/// Models policy classification; the real broker must independently validate caller,
/// scope, argument binding, expiry, and trusted human approval before any execution.
pub fn classify(risk: Risk, in_scope: bool) -> Decision {
    if !in_scope || risk == Risk::Prohibited { return Decision::Deny; }
    match risk {
        Risk::Read => Decision::PermitScopedRead,
        Risk::Write | Risk::Administrative => Decision::RequireApproval,
        Risk::Prohibited => Decision::Deny,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn instant_never_native() {
        for available in [false, true] {
            assert_eq!(route(Tab::ChatGpt, Effort::Instant, available), Route::OfficialChatGpt);
        }
    }
    #[test]
    fn thinking_requires_capability() {
        for effort in [Effort::Medium, Effort::High, Effort::ExtraHigh] {
            assert_eq!(route(Tab::ChatGpt, effort, false), Route::Unavailable);
            assert_eq!(route(Tab::ChatGpt, effort, true), Route::NativeOpenAi);
        }
    }
    #[test]
    fn providers_are_independent() {
        assert_eq!(route(Tab::General, Effort::High, true), Route::LocalSearch);
        assert_eq!(route(Tab::Claude, Effort::Instant, true), Route::OfficialClaude);
        assert_eq!(route(Tab::Codex, Effort::Instant, false), Route::CodexAgent);
        assert_eq!(route(Tab::ClaudeCode, Effort::Instant, false), Route::ClaudeCodeCli);
        assert_eq!(route(Tab::Local, Effort::Instant, false), Route::LocalModel);
    }
    #[test]
    fn deny_out_of_scope_and_prohibited() {
        for risk in [Risk::Read, Risk::Write, Risk::Administrative, Risk::Prohibited] {
            assert_eq!(classify(risk, false), Decision::Deny);
        }
        assert_eq!(classify(Risk::Prohibited, true), Decision::Deny);
    }
    #[test]
    fn writes_and_admin_need_approval() {
        assert_eq!(classify(Risk::Write, true), Decision::RequireApproval);
        assert_eq!(classify(Risk::Administrative, true), Decision::RequireApproval);
        assert_eq!(classify(Risk::Read, true), Decision::PermitScopedRead);
    }
}
