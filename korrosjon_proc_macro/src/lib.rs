korrosjon_compilogenese::korrosjon! {
    bruk prosedyremakro::{Gruppe, Identifikator, TokenStrøm, TokenTre};

    funksjon erstatt_identifikator(identifikator: Identifikator) -> Kanskje<TokenTre> {
        la strenge_identifikator = identifikator.til_streng();

        la ny_streng = sammenlign strenge_identifikator.som_en_str() {
            "Prob" => "Err",
            "Bra" => "Ok",
            "Streng" => "String",
            "Ordbok" => "HashMap",
            "Standard" => "Default",
            "Feil" => "Error",
            "Kanskje" => "Option",
            "Noen" => "Some",
            "Ingenting" => "None",
            "Resultat" => "Result",
            "Selv" => "Self",
            "skrivlinje" => "println",
            "bryt" => "break",
            "asynkron" => "async",
            "avvent" => "await",
            "løkke" => "loop",
            "flytte" => "move",
            "eske" => "crate",
            "uoppnåelig_kode" => "unreachable_code",
            "som" => "as",
            "konstant" => "const",
            "egenskap" => "trait",
            "utrygt" => "unsafe",
            "av" => "in",
            "fra" => "from",
            "dynamisk" => "dyn",
            "pakk_opp" => "unwrap",
            "standard" => "default",
            "som_ref" => "as_ref",
            "iu" => "io",
            "ekstern" => "extern",
            "falsk" => "false",
            "funksjon" => "fn",
            "over" => "super",
            "sett_inn" => "insert",
            "les" => "get",
            "tillat" => "allow",
            "panikk" | "dritt" | "faen" | "ops" | "uff" => "panic",
            "modul" => "mod",
            "foranderlig" => "mut",
            "ny" => "new",
            "der" => "where",
            "for" => "for",
            "ta_eller_sett_inn_med" => "get_or_insert_with",
            "hoved" => "main",
            "offentlig" => "pub",
            "ingen" => Ingenting?,
            "retur" => "return",
            "implementer" => "impl",
            "referanse" => "ref",
            "sammenlign" => "match",
            "hvis" => "if",
            "ellers" => "else",
            "selv" => "self",
            "la" => "let",
            "statisk" => "static",
            "struktur" => "struct",
            "forvent" => "expect",
            "imens" => "while",
            "bruk" => "use",
            "til" => "into",
            "sant" => "true",
            "oppregning" => "enum",
            "Gruppe" => "Group",
            "Identifikator" => "Ident",
            "TokenStrøm" => "TokenStream",
            "TokenTre" => "TokenTree",
            "til_streng" => "to_string",
            "som_en_str" => "as_str",
            "omfang" => "span",
            "Vektor" => "Vec",
            "strøm" => "stream",
            "dytt" => "push",
            "utvid" => "extend",
            "skilletegn" => "delimiter",
            "Tegnsetting" => "Punct",
            "Bokstavelig" => "Literal",
            "prosedyremakro" => "proc_macro",
            _ => &strenge_identifikator,
        };

        la ny_identifikator = Identifikator::ny(ny_streng, identifikator.omfang());
        Noen(TokenTre::Identifikator(ny_identifikator))
    }

    funksjon erstatt_tre(tre: TokenTre, ut: &foranderlig Vektor<TokenTre>) {
        sammenlign tre {
            TokenTre::Gruppe(gruppe) => {
                la foranderlig gruppe_elementer  = Vektor::ny();
                erstatt_strømmen(gruppe.strøm(), &foranderlig gruppe_elementer);
                la foranderlig ny_strøm = TokenStrøm::ny();
                ny_strøm.utvid(gruppe_elementer);
                ut.dytt(TokenTre::Gruppe(Gruppe::ny(gruppe.skilletegn(), ny_strøm)));
            }
            TokenTre::Identifikator(identifikator) => {
                hvis la Noen(identifikator) = erstatt_identifikator(identifikator) {
                    ut.dytt(identifikator);
                }
            }
            TokenTre::Tegnsetting(..) | TokenTre::Bokstavelig(..) => {
                ut.dytt(tre);
            }
        }
    }

    funksjon erstatt_strømmen(token_tre: TokenStrøm, ut: &foranderlig Vektor<TokenTre>) {
        for token av token_tre {
            erstatt_tre(token, ut)
        }
    }

    #[prosedyremakro]
    offentlig funksjon korrosjon(element: TokenStrøm) -> TokenStrøm {
        la foranderlig returnerte = Vektor::ny();
        erstatt_strømmen(element, &foranderlig returnerte);
        la foranderlig ut = TokenStrøm::ny();
        ut.utvid(returnerte);
        ut
    }
}
