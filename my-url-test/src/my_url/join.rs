use crate::prelude::*;

#[derive(Debug, Error)]
pub enum InvalidJoin {
    #[error(transparent)] InvalidUrl       (#[from] InvalidUrl       ),
    #[error(transparent)] InvalidScheme    (#[from] InvalidScheme    ),
    #[error(transparent)] SetPathError     (#[from] SetPathError     ),
    #[error(transparent)] SetQueryError    (#[from] SetQueryError    ),
    #[error(transparent)] SetFragmentError (#[from] SetFragmentError ),
    #[error(transparent)] CannotBeABase    (#[from] CannotBeABase    ),
    #[error("Other")]
    Other
}

#[derive(Debug, Error)]
#[error("TODO")]
pub struct CannotBeABase;

impl MyUrl {
    pub fn join(&mut self, value: &str) -> Result<bool, InvalidJoin> {
        println!("join");

        if self.cannot_be_a_base() {
            Err(CannotBeABase)?;
        }

        let start = value.bytes(). position(|b| b > 0x20 && b != 0x7F).unwrap_or(0);
        let end   = value.bytes().rposition(|b| b > 0x20 && b != 0x7F).map_or(0, |x| x + 1);

        let mut value = Cow::Borrowed(&value[start..end]);

        if value.bytes().any(|b| b == b'\t' || b == b'\n' || b == b'\r') {
            value.to_mut().retain(|c| c != '\t' && c != '\n' && c != '\r');
        }

        self.join_scheme_start(&value)
    }

    fn join_scheme_start(&mut self, value: &str) -> Result<bool, InvalidJoin> {
        println!("join_scheme_start");

        match value.bytes().next() {
            Some(b'a'..=b'z' | b'A'..=b'Z') if let Some(i) = value.bytes().position(|b| b == b':') && let Ok(scheme) = Scheme::new(unsafe {value.get_unchecked(..i)}) => {
                let rest = unsafe {value.get_unchecked(i+1..)};
                self.join_scheme(value, scheme, rest)
            },
            _ => self.join_no_scheme(value)
        }
    }

    fn join_no_scheme(&mut self, rest: &str) -> Result<bool, InvalidJoin> {
        println!("join_no_scheme");

        match self.path().starts_with('/') {
            true => match self.details.scheme.is_file() {
                true => self.join_file_no_scheme(rest),
                false => self.join_relative(rest)
            },
            false => match rest.starts_with('#') {
                true  => todo!(),
                false => Err(InvalidJoin::Other),
            }
        }
    }

    fn join_file_no_scheme(&mut self, rest: &str) -> Result<bool, InvalidJoin> {
        println!("join_file_no_scheme");

        *self = MyUrl::after_scheme("file".parse()?, rest)?;
        Ok(true)
    }

    fn join_file(&mut self, value: &str) -> Result<bool, InvalidJoin> {
        println!("join_file");

        *self = MyUrl::new(value)?;
        Ok(true)
    }

    fn join_scheme(&mut self, value: &str, scheme: Scheme<'_>, rest: &str) -> Result<bool, InvalidJoin> {
        println!("join_scheme");

        match scheme.r#type() {
            SchemeType::File           => self.join_file(value),
            SchemeType::SpecialNotFile => match self.details.scheme == scheme.details() {
                true  => self.join_sroa(value, rest),
                false => self.join_sas(value, rest),
            },
            SchemeType::NonSpecial => match rest.starts_with('/') {
                true  => self.join_poa(value, scheme, rest),
                false => self.join_opaque_path(value),
            },
        }
    }

    fn join_sas(&mut self, value: &str, rest: &str) -> Result<bool, InvalidJoin> {
        println!("join_sas");

        *self = MyUrl::new(value)?;
        Ok(true)
    }

    fn join_poa(&mut self, value: &str, scheme: Scheme<'_>, rest: &str) -> Result<bool, InvalidJoin> {
        println!("join_poa");

        if rest.starts_with("//") {
            *self = MyUrl::new(value)?;

            Ok(true)
        } else {
            self.join_path(value, scheme, rest)
        }
    }

    fn join_path(&mut self, value: &str, scheme: Scheme<'_>, rest: &str) -> Result<bool, InvalidJoin> {
        println!("join_path");

        match scheme.is_special() {
            true  => todo!(),
            false => {
                *self = MyUrl::new(value)?;
                Ok(true)
            }
        }
    }

    fn join_opaque_path(&mut self, value: &str) -> Result<bool, InvalidJoin> {
        println!("join_opaque_path");

        *self = MyUrl::new(value)?;
        Ok(true)
    }

    fn join_sroa(&mut self, value: &str, rest: &str) -> Result<bool, InvalidJoin> {
        println!("join_sroa");

        match rest.starts_with("//") {
            true => {
                *self = Self::new(value)?;

                Ok(true)
            },
            false => self.join_relative(rest)
        }
    }

    fn join_relative(&mut self, rest: &str) -> Result<bool, InvalidJoin> {
        println!("join_relative");

        match (rest.as_bytes(), self.details.scheme.is_special()) {
            ([b'/' | b'\\', ..], true) | ([b'/', ..], false) => self.join_relative_slash(&rest[1..]),
            _ => self.join_relative_not_slash(rest)
        }
    }

    fn join_relative_slash(&mut self, rest: &str) -> Result<bool, InvalidJoin> {
        println!("join_relative_slash");

        match (rest.as_bytes(), self.details.scheme.is_special()) {
            ([b'/' | b'\\', ..], true ) | ([b'/', ..], false) => self.join_authority(rest),
            _ => {
                let (p, q, f) = split_pqf(rest);

                let mut changed = self.set_path(p)?;

                if let Some(q) = q {changed |= self.set_query   (q)?;}
                if let Some(f) = f {changed |= self.set_fragment(f)?;}

                Ok(changed)
            }
        }
    }

    fn join_authority(&mut self, rest: &str) -> Result<bool, InvalidJoin> {
        println!("join_authority");

        *self = MyUrl::after_scheme(self.scheme().parse().expect("???"), rest)?;
        Ok(true)
    }

    fn join_relative_not_slash(&mut self, rest: &str) -> Result<bool, InvalidJoin> {
        println!("join_relative_not_slash");

        let mut changed = false;

        let (p, q, f) = split_pqf(rest);

        if !p.is_empty() {
            let mut x = self.path().to_string();
            x.truncate(x.bytes().rposition(|b| b == b'/').expect("???") + 1);
            x.push_str(p);
            changed |= self.set_path(x)?;
        }

        if let Some(q) = q {
            changed |= self.set_query(q)?;
        }

        if let Some(f) = f {
            changed |= self.set_fragment(f)?;
        }

        Ok(changed)
    }
}
