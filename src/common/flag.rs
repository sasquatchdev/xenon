/// Represents additional information on some
/// arbitrary data.
pub enum Flag<'a> {
  /// No additional information
  None,

  /// Optional, no action required.
  /// This includes negligable performance matters or
  /// style guidelines.
  Suggestion(&'a str),

  /// Still Optional, action reccomended.
  /// This includes major performance matters, dead code
  /// or known runtime issues.
  Warning(&'a str),

  /// Mandatory, action required.
  /// Anything more important than a warning. Contains
  /// lexical, syntactical and codegen errors.
  Error(&'a str)
}