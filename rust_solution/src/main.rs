#![feature(isolate_most_least_significant_one)]

trait MaybeValid {
    fn is_valid(&self) -> bool;
}
impl MaybeValid for [u8; 9] {
    fn is_valid(&self) -> bool {
        let mut occurance = [false; 9];
        for i in self {
            if *i < 1 {
                return false;
            }
            occurance[(*i - 1) as usize] = true
        }
        !occurance.contains(&false)
    }
}

#[derive(Copy, Clone)]
struct CandidateSet(u16);
impl CandidateSet {
    fn new() -> Self {
        CandidateSet(0b111111111)
    }
}

#[derive(Copy, Clone)]
struct CandidateSetIterator(u16);

impl Iterator for CandidateSetIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        } else {
            let set_flag = self.0.isolate_least_significant_one();
            self.0 ^= set_flag;
            Some(set_flag.ilog2() as u8 + 1)
        }
    }
}

impl IntoIterator for CandidateSet {
    type Item = u8;
    type IntoIter = CandidateSetIterator;

    fn into_iter(self) -> Self::IntoIter {
        CandidateSetIterator(self.0)
    }
}

impl core::ops::BitAndAssign<u16> for CandidateSet {
    fn bitand_assign(&mut self, rhs: u16) {
        self.0 &= rhs;
    }
}

impl core::ops::BitOrAssign<u16> for CandidateSet {
    fn bitor_assign(&mut self, rhs: u16) {
        self.0 |= rhs;
    }
}

impl core::ops::BitAnd<CandidateSet> for CandidateSet {
    type Output = CandidateSet;

    fn bitand(self, rhs: CandidateSet) -> Self::Output {
        CandidateSet(self.0 & rhs.0)
    }
}

pub struct Sudoku {
    grid: [u8; 41],
    row_candidates: [CandidateSet; 9],
    col_candidates: [CandidateSet; 9],
    grid_candidates: [CandidateSet; 9],
}

impl Sudoku {
    fn get(&self, index: u8) -> u8 {
        let val = self.grid[(index >> 1) as usize];
        if index & 1 == 1 {
            val >> 4
        } else {
            val & 0b1111
        }
    }

    fn is_missing(&self, index: u8) -> bool {
        self.get(index) == 0
    }

    fn set(&mut self, index: u8, val: u8) {
        let oldval;
        let oldvalpair = &mut self.grid[(index >> 1) as usize];
        let row_candidate_set = &mut self.row_candidates[Self::row_index(index) as usize];
        let col_candidate_set = &mut self.col_candidates[Self::col_index(index) as usize];
        let grid_candidate_set = &mut self.grid_candidates[Self::grid_index(index) as usize];

        if index & 1 == 1 {
            oldval = *oldvalpair >> 4;
            *oldvalpair = *oldvalpair & 0b1111 | val << 4;
        } else {
            oldval = *oldvalpair & 0b1111;
            *oldvalpair = (*oldvalpair & 0b11110000) | val;
        }

        if oldval != 0 {
            let bit_filter = 1 << (oldval - 1);
            *row_candidate_set |= bit_filter;
            *col_candidate_set |= bit_filter;
            *grid_candidate_set |= bit_filter;
        }
        if val != 0 {
            let bit_filter = !(1 << (val - 1));
            *row_candidate_set &= bit_filter;
            *col_candidate_set &= bit_filter;
            *grid_candidate_set &= bit_filter;
        }
    }

    /*
    00 01 02 03 04 05 06 07 08
    09 10 11 12 13 14 15 16 17
    18 19 20 21 22 23 24 25 26
    27 28 29 30 31 32 33 34 35
    36 37 38 39 40 41 42 43 44
    45 46 47 48 49 50 51 52 53
    54 55 56 57 58 59 60 61 62
    63 64 65 66 67 68 69 70 71
    72 73 74 75 76 77 78 79 80
    */

    fn grid_index(index: u8) -> u8 {
        let row_num = index / 27;
        let col_num = (index % 9) / 3;
        row_num * 3 + col_num
    }

    fn row_index(index: u8) -> u8 {
        index / 9
    }

    fn col_index(index: u8) -> u8 {
        index % 9
    }

    fn get_candidates(&self, index: u8) -> impl Iterator<Item = u8> + 'static {
        let row_candidate_set = self.row_candidates[Self::row_index(index) as usize];
        let col_candidate_set = self.col_candidates[Self::col_index(index) as usize];
        let grid_candidate_set = self.grid_candidates[Self::grid_index(index) as usize];
        let subset = col_candidate_set & row_candidate_set & grid_candidate_set;
        subset.into_iter()
    }
}

impl MaybeValid for Sudoku {
    fn is_valid(&self) -> bool {
        for i in 0..9 {
            if ![
                self.get(i * 9),
                self.get(i * 9 + 1),
                self.get(i * 9 + 2),
                self.get(i * 9 + 3),
                self.get(i * 9 + 4),
                self.get(i * 9 + 5),
                self.get(i * 9 + 6),
                self.get(i * 9 + 7),
                self.get(i * 9 + 8),
            ]
            .is_valid()
            {
                return false;
            }

            if ![
                self.get(i),
                self.get(i + 9),
                self.get(i + 9 * 2),
                self.get(i + 9 * 3),
                self.get(i + 9 * 4),
                self.get(i + 9 * 5),
                self.get(i + 9 * 6),
                self.get(i + 9 * 7),
                self.get(i + 9 * 8),
            ]
            .is_valid()
            {
                return false;
            }
        }

        for i in [0, 3, 6, 27, 30, 33, 54, 57, 60] {
            if ![
                self.get(i),
                self.get(i + 1),
                self.get(i + 2),
                self.get(i + 9),
                self.get(i + 10),
                self.get(i + 11),
                self.get(i + 18),
                self.get(i + 19),
                self.get(i + 20),
            ]
            .is_valid()
            {
                return false;
            }
        }

        return true;
    }
}

impl From<[u8; 81]> for Sudoku {
    fn from(v: [u8; 81]) -> Self {
        let mut sudoku = Sudoku {
            grid: [0; _],
            row_candidates: [CandidateSet::new(); _],
            col_candidates: [CandidateSet::new(); _],
            grid_candidates: [CandidateSet::new(); _],
        };
        for (k, v) in v.into_iter().enumerate() {
            if v != 0 {
                sudoku.set(k as u8, v);
            }
        }
        sudoku
    }
}

impl From<[u8; 41]> for Sudoku {
    fn from(v: [u8; 41]) -> Self {
        let mut sudoku = Sudoku {
            grid: [0; _],
            row_candidates: [CandidateSet::new(); _],
            col_candidates: [CandidateSet::new(); _],
            grid_candidates: [CandidateSet::new(); _],
        };
        for (k, v) in v.into_iter().enumerate() {
            let v1 = v & 0b1111;
            let v2 = v >> 4;
            if v1 != 0 {
                sudoku.set((k*2) as u8, v1);
            }
            if v2 != 0 && k != 40 {
                sudoku.set((k*2+1) as u8, v2);
            }
        }
        sudoku
    }
}

impl From<&str> for Sudoku {
    fn from(value: &str) -> Self {
        let mut sudoku = Sudoku {
            grid: [0; _],
            row_candidates: [CandidateSet::new(); _],
            col_candidates: [CandidateSet::new(); _],
            grid_candidates: [CandidateSet::new(); _],
        };
        for (k, v) in value.as_bytes().iter().enumerate() {
            let digit = *v - '0' as u8;
            if digit != 0 as u8 {
                sudoku.set(k as u8, digit);
            }
        }
        sudoku
    }
}

impl core::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for i in 0..9 {
            "\n".fmt(f)?;
            for j in 0..9 {
                let val = self.get(i * 9 + j);
                if val == 0 {
                    "?".fmt(f)?;
                } else {
                    val.fmt(f)?;
                }
                ", ".fmt(f)?;
            }
        }
        Ok(())
    }
}

pub fn solve(sudoku: &mut Sudoku, index: u8, callback: &impl Fn(&Sudoku)) {
    if index == 81 {
        callback(sudoku);
        return;
    }
    if sudoku.is_missing(index) {
        for candidate_val in sudoku.get_candidates(index) {
            sudoku.set(index, candidate_val);
            solve(sudoku, index + 1, callback);
        }
        sudoku.set(index, 0);
    } else {
        solve(sudoku, index + 1, callback);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        test_helper("519872436643159827782463195495217368127386954836945271354791682268534719971028543");
        test_helper("015369874748215936369748521497182365653497218182653749934871652526934187871526493");
        test_helper("675491382238675914091238756984352167716984523352716849563147298147829635829563471");
        test_helper("206473819819625734734981256567834192192756348348219567925367481481592673673148925");
        test_helper("186293547540186293293547186435871962871962435962435871758619324324758619619324758");
        test_helper("613574928475298136892316754549182673281763495367945812734859261120437589958621347");
        test_helper("459821367281367549637549821765498213123075498948213675892136754316754982574982136");
        test_helper("914562078256873941387149265549628137173495826862731594695287413728314659431956782");
        test_helper("186293745547681293392745681658912437219437856734856912961324578403578169875169324");
        test_helper("070030000000012000002005019050020073700000105140000906200054008630109502005267300");
        test_helper("924000305010900280080007916200090700000704021030100590602010007500870100000006000");
        test_helper("459063001000040000000095030000802640940000710607910520001600070074080000800030102");
        test_helper("000070840080004027040002610090300750530006480000040396900081004060500900872000000");
        test_helper("000489700000000809090070020000097000100804902028100003536008010082001305009003608");
        test_helper("450063280300200000821590000703082050000037020000045000180009062000320010002050007");
        test_helper("400703080000000406680400009005300000093820005064005300070530600031679050500280000");
        test_helper("004680000300205419050930000230508001040190850000760090090020537000000006600050020");
        test_helper("040000000000020740230400608020057900980000100001068000000502067800714092500600314");
        test_helper("608040050300506097905000300030095208590700640004063010007001004800604500000000060");
        test_helper("107008000693000020048000000980301460360052781500806230030500008700004006010000000");
        test_helper("675321984312498576489657213967513042531284769248976135753142698896735421124869357");
        test_helper("851276394726493185943581672217649853469358721538127946102764539674935218395812467");
        test_helper("519728436728634915634519827347196582196285743285347691963851274851472369472963108");
        test_helper("849673512637015498251894376476321985598467123312589764185946237964732851723158649");
        test_helper("374289561298561347516347289985136724742958136163724958637492815851673492429815670");
        test_helper("613429875924758631857316942279835164538641297146972580782563419491287356365194728");
        test_helper("186352794235947186094861235342679851851423967967518342619285473473196528528734619");
        test_helper("914738526783625049652941837495317682268459713371286954139872465546193278827564391");
        test_helper("140627359359148627627359148296583471471296583583471296815764932764932815932815764");
        test_helper("694513287531782946728649315172896534869435721453127698945371862317208459286954173");
        test_helper("369241078758396124014785639876913245931452786425867391683129457192574863547638912");
        test_helper("274638951638159472159274836782396145396541087541782693867913524425807319913425768");
        test_helper("357194628419826375682573491061387954738459216945012783593241867124768539876935142");
        test_helper("624579183795183462831462579573218946182946357469357218358621794216794835907830621");
        test_helper("758421396963087214142639875314968527275143689896752431427316958631895742589274160");
        test_helper("624579381597381642318642579839416725052893416461725893143067958276958134985134267");
        test_helper("471368925925047683683592471816923754239475816754681239148236597597814362362759048");
        test_helper("436987152978251364215463789182345697769128543354796821891532476647819235523074908");
        test_helper("231407598859123746674985312346879025798251463512634987925316874487592631163748259");
        test_helper("593280761284617359617593428468751932329468175751329846942876513876135294135942607");
        test_helper("624831597801795426795624138257346819918257643340918752472163985163589274589472361");
        test_helper("152038746398647215467512839571293468923864157084751392219386574836475921745129683");
        test_helper("697825413341976825582413076925138764813764259476259138164597382238601597759382641");
        test_helper("218435796345796128976128430459672813183549672762813549597261384834907261621384957");
        test_helper("539824176761359428284671953690432781342187569817965034478516392156293847923748615");
        test_helper("539824176428671539176935428853246917917350642642719853285467391760193285391582764");
        test_helper("524719638368542971791086254870423519159867342432195786617238495945671823283954167");
        test_helper("957812634281306957634579281468937125793251468025684793519428376842763519376195842");
        test_helper("134258697967134528258967314813725906496013275725496183349581762672349851581672439");
    }

    fn test_helper(sudoku_str: &str) {
        let mut sudoku: Sudoku = sudoku_str.into();
        let did_solve = core::cell::Cell::new(false);
        assert!(!sudoku.is_valid());
        solve(&mut sudoku, 0, &|s| {
            assert!(s.is_valid());
            did_solve.set(true);
        });
        assert!(did_solve.get());
    }
}

fn main() {
    let mut sudoku: Sudoku =
        "070030000000012000002005019050020073700000105140000906200054008630109502005267300".into();
    assert!(!sudoku.is_valid());
    println!("Problem: {}", sudoku);
    solve(&mut sudoku, 0, &|s| {
        assert!(s.is_valid());
        println!("solved: {}", &s);
    });
}
