/// Represents additional information on some
/// arbitrary data.
#[derive(Debug, Clone)]
pub enum Flag {
  /// No additional information
  None,

  /// Optional, no action required.
  /// This includes negligable performance matters or
  /// style guidelines.
  Suggestion(String),

  /// Still Optional, action reccomended.
  /// This includes major performance matters, dead code
  /// or known runtime issues.
  Warning(String),

  /// Mandatory, action required.
  /// Anything more important than a warning. Contains
  /// lexical, syntactical and codegen errors.
  Error(String)
}