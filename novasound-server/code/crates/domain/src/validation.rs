use std::fmt;

/// A stable, field-level user input validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
    pub field: String,
    pub code: String,
    pub message: String,
}

impl ValidationIssue {
    #[must_use]
    pub fn new(
        field: impl Into<String>,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            field: field.into(),
            code: code.into(),
            message: message.into(),
        }
    }
}

/// Validation failures collected while inspecting one command.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ValidationErrors {
    issues: Vec<ValidationIssue>,
}

impl ValidationErrors {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, issue: ValidationIssue) {
        self.issues.push(issue);
    }

    pub fn extend(&mut self, errors: Self) {
        self.issues.extend(errors.issues);
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.issues.is_empty()
    }

    #[must_use]
    pub fn issues(&self) -> &[ValidationIssue] {
        &self.issues
    }
}

impl From<ValidationIssue> for ValidationErrors {
    fn from(issue: ValidationIssue) -> Self {
        Self {
            issues: vec![issue],
        }
    }
}

impl fmt::Display for ValidationErrors {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut issues = self.issues.iter();
        let Some(first) = issues.next() else {
            return formatter.write_str("Validation failed");
        };

        write!(formatter, "{}", first.message)?;
        for issue in issues {
            write!(formatter, "; {}", issue.message)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationErrors {}
