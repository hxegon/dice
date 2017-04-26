use std::str::FromStr;
use std::fmt;

/// Store roll parameters
///
/// ** Parameters: **
/// - Count: number of dice you want to roll
/// - Sides: number of sides to each dice
#[derive(Eq, PartialEq)]
pub struct RollCmd {
    count: u32,
    sides: u32,
}

impl RollCmd {
    // Construct a new RollCmd. Count, then Sides.
    pub fn new(c: u32, s: u32) -> RollCmd {
        RollCmd { count: c, sides: s }
    }

    /// Generates a new RollResult based on a RollCmd.
    ///
    /// Each RollCmd can be used repeatedly; this function will generate new
    /// RollResults each time.
    /// Because this is a higher order function it's up to the caller to provide
    /// an appropriate 'random value of range' function.
    ///
    /// # Examples
    ///
    /// Here we provide result with a max function, returning the highest
    /// possible value for each roll.
    /// ```
    /// use rcmd::RollCmd;
    /// let cmd = RollCmd::new(2, 6);
    /// let result = cmd.result(|max| max);
    /// assert!([6, 6] == result.values());
    /// ```
    pub fn result<F: FnMut(u32) -> u32>(&self, mut f: F) -> RollResult {
        RollResult((0..self.count).map(|_| f(self.sides)).collect())
    }
}

impl FromStr for RollCmd {
    type Err = String;

    /// Convert a string to a Result with a RollCmd struct.
    fn from_str(s: &str) -> Result<RollCmd, <RollCmd as FromStr>::Err> {
        let split: Vec<u32> = s.split('d').filter_map(|n| n.parse().ok()).collect();
        // Based on number of items grabbed by split, Ok(RollCmd) or Err
        match split.len() {
        // Could do this cleaner with a slice pattern, but that would require nightly :\
            2 => {
                let (count, sides) = (split[0], split[1]);
                Ok(RollCmd::new(count, sides))
            }
            1 => { 
                let sides = split[0];
                Ok(RollCmd::new(1, sides))
            }
            _ => Err(format!("Invalid RollCmd: {}", s))
        }
        
    }
}

/// A vector of u32 representing the result of a RollCmd.
///
/// RollResult allows us to provide specialized function impementations for
/// dealing with roll results.
pub struct RollResult(Vec<u32>);

impl RollResult {
    /// Returns an iterator over the result of a roll.
    ///
    /// Basically returns an iterator on the underlying vector.
    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, u32> {
        self.0.iter()
    }

    pub fn total(&self) -> u32 { // maybe change to u64?
        // TODO: Does this repeat RollResult::iter?
        self.0.iter().fold(0, |a, b| a + b)
    }

    /// Returns the individual rolls as a slice.
    ///
    /// Basically unwraps the RollResult into it's underlying Vec<u32>
    pub fn values(&self) -> &[u32] {
        &self.0
    }
}

impl fmt::Display for RollResult {
    /// Implement Display for Rollresult.
    ///
    /// # Examples
    /// ```
    /// use rcmd::RollResult;
    /// let result = RollResult(vec![2, 3, 3]);
    /// assert!(result.to_string() == "2, 3, 3 (Total: 8)");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let as_strings: Vec<_> = self.iter().map(|n| n.to_string()).collect();
        write!(f, "{} (Total: {})", as_strings.join(", "), self.total())
    }
}

#[cfg(test)]
mod rollcmd_tests {
    use super::*;

    // FromStr tests
    #[test]
    fn can_parse_full_rollcmds() {
        let cmd = RollCmd::new(2, 6);
        assert!(cmd == "2d6".parse().unwrap());
    }

    #[test]
    fn can_parse_short_rollcmds() {
        let cmd = RollCmd::new(1, 6);
        assert!(cmd == "6".parse().unwrap());
    }
}
