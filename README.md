# Lan Chat ![crates.io](https://img.shields.io/crates/v/gregex.svg) ![Build Passing](https://github.com/Saphereye/gregex/actions/workflows/ci.yml/badge.svg)

Gregex is a regular expression solver which utilizes Non-deterministic Finite Automata (NFA) to simulate the input strings.

## Usage

```rust
use::gregex::regex;

fn main() {
    let regex = regex('(a.b)*');
    assert!(regex.simulate('abab'));
}
```

## Theory
The project uses [Glushkov's construction algorithm](https://en.wikipedia.org/wiki/Glushkov%27s_construction_algorithm) for creating the NFA.

The pipeline can be summarised as below
[![](https://mermaid.ink/img/pako:eNptkMFqwzAQRH9F7KmF5Ad8KCRxnBhKKdFRymGx146oJQVlRVNC_r2y5YML3dPMvFlJ6AGNbwkK6Ab_3VwwsHg_aSfSbNSJeroLycG4_izW67etql1n7oK9-AyU1DlXtyMUu3mBA9EMdhMoVW4LSfwH7JWM3X-gUhU27MNtQcqJHNRHtZmTfU6yqZbm-FK7a-T57a8TqjM6LE2tpLFxwHRVOnPMYAWWgkXTpk95jIkGvpAlDUWSLYYvDdo9Uw8je_njGig4RFpBvLbIVBrsA1ooOhxu9PwFzJVo_Q?type=png)](https://mermaid.live/edit#pako:eNptkMFqwzAQRH9F7KmF5Ad8KCRxnBhKKdFRymGx146oJQVlRVNC_r2y5YML3dPMvFlJ6AGNbwkK6Ab_3VwwsHg_aSfSbNSJeroLycG4_izW67etql1n7oK9-AyU1DlXtyMUu3mBA9EMdhMoVW4LSfwH7JWM3X-gUhU27MNtQcqJHNRHtZmTfU6yqZbm-FK7a-T57a8TqjM6LE2tpLFxwHRVOnPMYAWWgkXTpk95jIkGvpAlDUWSLYYvDdo9Uw8je_njGig4RFpBvLbIVBrsA1ooOhxu9PwFzJVo_Q)