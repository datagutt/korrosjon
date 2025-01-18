korrosjon_compilogenese::korrosjon! {
    bruk prosedyremakro::{Gruppe, Identifikator, TokenFlyt, TokenTre};

    funksjon erstatt_identifikator(identifikator: Identifikator) -> Kanskje<TokenTre> {
        la strenge_identifikator = identifikator.til_streng();

        la ny_streng = sammenlign strenge_identifikator.som_en_streng() {
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
            "TokenFlyt" => "TokenStream",
            "TokenTre" => "TokenTree",
            "til_streng" => "to_string",
            "som_en_streng" => "as_str",
            "omfang" => "span",
            "Vektor" => "Vec",
            "flyt" => "stream",
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

    funksjon erstatt_tre(tre: TokenTre, sortering: &foranderlig Vektor<TokenTre>) {
        sammenlign tre {
            TokenTre::Gruppe(gruppe) => {
                la foranderlig gruppe_elementer  = Vektor::ny();
                erstatt_flyten(gruppe.stream(), &foranderlig gruppe_elementer);
                la foranderlig ny_flyt = TokenFlyt::ny();
                ny_flyt.utvid(gruppe_elementer);
                sortering.dytt(TokenTre::Gruppe(Gruppe::ny(gruppe.skilletegn(), ny_flyt)));
            }
            TokenTre::Identifikator(identifikator) => {
                hvis la Noen(identifikator) = erstatt_identifikator(identifikator) {
                    sortering.dytt(identifikator);
                }
            }
            TokenTre::Tegnsetting(..) | TokenTre::Bokstavelig(..) => {
                sortering.dytt(tre);
            }
        }
    }

    funksjon erstatt_flyten(token_tre: TokenFlyt, sortering: &foranderlig Vektor<TokenTre>) {
        for token av token_tre {
            erstatt_tre(token, sortering)
        }
    }

    #[prosedyremakro]
    offentlig funksjon korrosjon(element: TokenFlyt) -> TokenFlyt {
        la foranderlig returnerte = Vektor::ny();
        erstatt_flyten(element, &foranderlig returnerte);
        la foranderlig sortering = TokenFlyt::ny();
        sortering.utvid(returnerte);
        sortering
    }
}
