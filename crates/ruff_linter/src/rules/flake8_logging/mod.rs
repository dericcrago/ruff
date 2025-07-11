//! Rules from [flake8-logging](https://pypi.org/project/flake8-logging/).
pub(crate) mod rules;

#[cfg(test)]
mod tests {
    use std::path::Path;

    use anyhow::Result;
    use test_case::test_case;

    use crate::assert_diagnostics;
    use crate::registry::Rule;
    use crate::settings::LinterSettings;
    use crate::test::test_path;

    #[test_case(Rule::DirectLoggerInstantiation, Path::new("LOG001.py"))]
    #[test_case(Rule::InvalidGetLoggerArgument, Path::new("LOG002.py"))]
    #[test_case(Rule::ExceptionWithoutExcInfo, Path::new("LOG007.py"))]
    #[test_case(Rule::UndocumentedWarn, Path::new("LOG009.py"))]
    #[test_case(Rule::LogExceptionOutsideExceptHandler, Path::new("LOG004_0.py"))]
    #[test_case(Rule::LogExceptionOutsideExceptHandler, Path::new("LOG004_1.py"))]
    #[test_case(Rule::ExcInfoOutsideExceptHandler, Path::new("LOG014_0.py"))]
    #[test_case(Rule::ExcInfoOutsideExceptHandler, Path::new("LOG014_1.py"))]
    #[test_case(Rule::RootLoggerCall, Path::new("LOG015.py"))]
    fn rules(rule_code: Rule, path: &Path) -> Result<()> {
        let snapshot = format!("{}_{}", rule_code.noqa_code(), path.to_string_lossy());
        let diagnostics = test_path(
            Path::new("flake8_logging").join(path).as_path(),
            &LinterSettings::for_rule(rule_code),
        )?;
        assert_diagnostics!(snapshot, diagnostics);
        Ok(())
    }
}
