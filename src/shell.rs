pub fn init_bash() -> &'static str {
    r#"
_smart_cd_hook() {
    smart-cd add "$(pwd)"
}
PROMPT_COMMAND="_smart_cd_hook${PROMPT_COMMAND:+;$PROMPT_COMMAND}"

z() {
    local result
    result="$(smart-cd query "$@" 2>/dev/tty)"
    if [ -n "$result" ]; then
        cd "$result"
    fi
}
"#
}

pub fn init_powershell() -> &'static str {
    r#"
function global:z {
    $result = smart-cd query @args 2>&1
    if ($LASTEXITCODE -eq 0 -and $result) {
        Set-Location $result
    }
}

$global:_smart_cd_orig_prompt = $function:prompt

function global:prompt {
    smart-cd add (Get-Location).Path | Out-Null
    & $global:_smart_cd_orig_prompt
}
"#
}

pub fn init_fish() -> &'static str {
    r#"
function _smart_cd_hook --on-variable PWD
    smart-cd add "$PWD"
end

function z
    set result (smart-cd query $argv 2>/dev/tty)
    if test -n "$result"
        cd $result
    end
end
"#
}

pub fn init_cmd() -> &'static str {
    r#"
@echo off
:: smart-cd cmd integration
:: Add the following to your AutoRun registry key or batch profile:
::   doskey z=for /f "delims=" %%i in ('smart-cd query $*') do cd /d %%i
::
:: Directory tracking requires a scheduler or manual hook; cmd has no built-in PROMPT_COMMAND.
:: Simplest workaround: wrap cd with a batch function in your profile.

doskey z=for /f "delims=" %%i in ('smart-cd query $*') do cd /d %%i
"#
}
