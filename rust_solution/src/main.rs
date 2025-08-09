#![feature(isolate_most_least_significant_one)]

const MULTITHREADING_DEBUG: bool = true;

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

impl CandidateSetIterator {
    fn empty() -> Self {
        CandidateSetIterator(0)
    }

    fn fixed() -> Self {
        CandidateSetIterator(u16::MAX)
    }

    fn is_fixed(&self) -> bool {
        self.0 == u16::MAX
    }

    fn is_empty(&self) -> bool {
        self.0 == 0
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

#[derive(Clone)]
pub struct Sudoku {
    grid: [u8; 41],
    row_candidates: [CandidateSet; 9],
    col_candidates: [CandidateSet; 9],
    grid_candidates: [CandidateSet; 9],
}

impl PartialEq for Sudoku {
    fn eq(&self, other: &Self) -> bool {
        self.grid == other.grid
    }
}
impl Eq for Sudoku {}

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

    fn get_candidates(&self, index: u8) -> CandidateSet {
        let row_candidate_set = self.row_candidates[Self::row_index(index) as usize];
        let col_candidate_set = self.col_candidates[Self::col_index(index) as usize];
        let grid_candidate_set = self.grid_candidates[Self::grid_index(index) as usize];
        col_candidate_set & row_candidate_set & grid_candidate_set
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
                sudoku.set((k * 2) as u8, v1);
            }
            if v2 != 0 && k != 40 {
                sudoku.set((k * 2 + 1) as u8, v2);
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

struct SingleLineDisplayAdaptor<'a, T>(&'a T);
impl core::fmt::Display for SingleLineDisplayAdaptor<'_, Sudoku> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for i in 0..9 {
            // "\n".fmt(f)?;
            for j in 0..9 {
                let val = self.0.get(i * 9 + j);
                if val == 0 {
                    "?".fmt(f)?;
                } else {
                    val.fmt(f)?;
                }
                // ", ".fmt(f)?;
            }
        }
        Ok(())
    }
}

use std::sync::{Mutex, RwLock, Condvar};
use std::thread;
use core::marker::PhantomData;

struct SharedContext<SOLFN> {
    current_problem_index: i32,
    current_problem: Sudoku,
    solution_callback: Option<SOLFN>, //None value signifies that problem is solved
}

struct Solver<'a, SOLFN>{
    shared_context: &'a Mutex<SharedContext<SOLFN>>
}

impl<'a, SOLFN> Solver<'a, SOLFN>
where SOLFN: Fn(&Sudoku) -> bool + std::marker::Send
{
    fn solve(&mut self, sudoku: &mut Sudoku, callback: SOLFN) {
        {
            let mut shared_context = self.shared_context.lock().unwrap();
            shared_context.current_problem_index += 1;
            shared_context.current_problem = sudoku.clone();
            shared_context.solution_callback = Some(callback);
        }
        if MULTITHREADING_DEBUG {
            println!("Main thread is {:?}", thread::current().id());
        }
        multithreaded_helper::<false, _>(self.shared_context);
    }
}

fn multithreaded_helper<const STAY_ALIVE: bool, SOLFN: Fn(&Sudoku) -> bool + std::marker::Send>(shared_context: &Mutex<SharedContext<SOLFN>>) {
    let mut local_last_known_problem_index;
    let mut local_last_known_problem;
    loop {
        if MULTITHREADING_DEBUG {
            println!("Thread {:?} is checking for new tasks...", thread::current().id());
        }
        {
            let shared_context = shared_context.lock().unwrap();
            if shared_context.current_problem_index == -1 {
                if MULTITHREADING_DEBUG {
                    println!("Thread {:?} acknowledged order to shut down", thread::current().id());
                }
                break;
            }
            if shared_context.solution_callback.is_none() {
                if MULTITHREADING_DEBUG {
                    println!("Thread {:?} found no new tasks", thread::current().id());
                }
                thread::yield_now();
                if !STAY_ALIVE {
                    if MULTITHREADING_DEBUG {
                        println!("Thread {:?} has !STAY_ALIVE, yielding control to caller.", thread::current().id());
                    }
                    break;
                }
                continue;
            }
            local_last_known_problem_index = shared_context.current_problem_index;
            local_last_known_problem = shared_context.current_problem.clone();
            if MULTITHREADING_DEBUG {
                println!("Thread {:?} found work, will start work on {}", thread::current().id(), local_last_known_problem_index);
            }
        }

        solve_single_thread::<false>(&mut local_last_known_problem, |solved_sudoku| {
            let mut shared_context = shared_context.lock().unwrap();
            match &shared_context.solution_callback {
                Some(callback) if shared_context.current_problem_index == local_last_known_problem_index && callback(solved_sudoku) => {
                    //We solved the current problem, which was unsolved
                    //Mark it solved, and cancel the current solve calc
                    if MULTITHREADING_DEBUG {
                        println!("Thread {:?} SUCCESS has solved problem {}", thread::current().id(), local_last_known_problem_index);
                    }

                    shared_context.solution_callback = None;
                    true
                }
                _ => {
                    let should_stop = shared_context.solution_callback.is_none() || shared_context.current_problem_index != local_last_known_problem_index;
                    if MULTITHREADING_DEBUG {
                        if should_stop {
                            println!("Thread {:?} CANCELLING problem {} [via on_solved]", thread::current().id(), local_last_known_problem_index);
                        } else {
                            println!("Thread {:?} CONTINUING,[via on_solved] for problem {}", thread::current().id(), local_last_known_problem_index);
                        }
                    }
                    should_stop
                }
            }
        }, || {
            let shared_context = shared_context.lock().unwrap();
            let should_stop = shared_context.solution_callback.is_none() || shared_context.current_problem_index != local_last_known_problem_index;
            if MULTITHREADING_DEBUG {
                if should_stop {
                    println!("Thread {:?} CANCELLING problem {} [via is_cancelled]", thread::current().id(), local_last_known_problem_index);
                } else {
                    println!("Thread {:?} CONTINUING,[via is_cancelled] for problem {}", thread::current().id(), local_last_known_problem_index);
                }
            }
            should_stop
        },
            |x| x as u8,
        );
    }
}

fn with_multithreaded_solver<T, SOLFN: Fn(&Sudoku) -> bool + std::marker::Send>(solving_callback: impl Fn (&mut Solver<SOLFN>) -> T) -> T {
    let mut ret_val: Option<T> = None;

    let shared_context = Mutex::new(SharedContext{current_problem_index: 0, current_problem: Sudoku::from("0"), solution_callback: None as Option<SOLFN> });
    let mut solver = Solver { shared_context: &shared_context };
    thread::scope(|scope| {
        scope.spawn(|| {
            if MULTITHREADING_DEBUG {
                println!("Spawned helper thread {:?}", thread::current().id());
            }

            multithreaded_helper::<true, _>(&shared_context);
        });

        ret_val = Some(solving_callback(&mut solver));
        let mut shared_context = shared_context.lock().unwrap();
        shared_context.current_problem_index = -1;
    });
    ret_val.unwrap()
}

// use std::sync::{Mutex, RwLock, Condvar};
// use std::thread;
// use core::marker::PhantomData;
// struct MultiThreadedSolver<const THREAD_COUNT: usize, const DEBUG: bool> {
//     current_problem: RwLock<(i32, Sudoku)>,
//     notifier: Condvar,
//     notifier_mutex: Mutex<()>,
// }
// impl<const THREAD_COUNT: usize, const DEBUG: bool> MultiThreadedSolver<THREAD_COUNT, DEBUG> {
//     fn start<'a, 'scope>(&'a mut self, thread_scope: &thread::Scope<'scope, 'a>)
//     where 'a : 'scope
//     {
//         let handle = thread_scope.spawn(|| {
//             let mut current_problem_number = 0;
//             loop {
//                 let (upsteam_problem_number, upstream_sudoku) = &*((self.current_problem).read().unwrap());
//                 if *upsteam_problem_number == 0 {
//                     self.notifier.wait(self.notifier_mutex.lock().unwrap());
//                 } else {
//                     // solve_single_thread::<DEBUG>(&mut upstream_sudoku.clone(), &|| {
//                     //         let write = solver.current_problem.write().unwrap();

//                     //     }, &|| {

//                     //     },
//                     //     &|x| x as u8
//                     // );
//                 }
//             }
//         });
//     }

//     fn new() -> Self {
//         let notifier = std::sync::Condvar::new();
//         let notifier_mutex = std::sync::Mutex::new(());
//         let rw_lock = RwLock::new((0, Sudoku::from("")));
//         MultiThreadedSolver{current_problem: rw_lock, notifier, notifier_mutex}
//         // let mut solver = ;
//         // scope.spawn(|| {
//         //     let mut current_problem_number = 0;
//         //     loop {
//         //         let (upsteam_problem_number, upstream_sudoku) = &*((&rw_lock).read().unwrap());
//         //         if *upsteam_problem_number == 0 {
//         //             notifier.wait(notifier_mutex.lock().unwrap());
//         //         } else {
//         //             // solve_single_thread::<DEBUG>(&mut upstream_sudoku.clone(), &|| {
//         //             //         let write = solver.current_problem.write().unwrap();

//         //             //     }, &|| {

//         //             //     },
//         //             //     &|x| x as u8
//         //             // );
//         //         }
//         //     }
//         // });
//         // solver
//     }
// }

// pub fn solve<const DEBUG: bool>(sudoku: &mut Sudoku, callback: impl Fn(&Sudoku) -> bool + std::marker::Sync) {
//     let did_solve = std::sync::Mutex::new(false);
//     let is_cancelled = || {
//         let did_solve = did_solve.lock().unwrap();
//         *did_solve
//     };

//     fn make_callback_wrapper(did_solve: &std::sync::Mutex<bool>, callback: impl Fn(&Sudoku) -> bool + std::marker::Sync) -> impl Fn(&Sudoku) -> bool + std::marker::Sync {
//         |x| {
//             let mut did_solve = did_solve.lock().unwrap();

//             if !*did_solve && callback(x) {
//                 *did_solve = true;
//                 true
//             } else {
//                 false
//             }
//         }
//     }

//     std::thread::scope(|scope| {
//         scope.spawn(|| {
//             solve_single_thread::<DEBUG>(&mut sudoku.clone(), &make_callback_wrapper(&did_solve, callback), &is_cancelled, &|x| (80-x) as u8);
//         });
//         scope.spawn(|| {
//             solve_single_thread::<DEBUG>(&mut sudoku.clone(), &make_callback_wrapper(&did_solve, callback), &is_cancelled, &|x| [20, 66, 78, 77, 39, 68, 57, 69, 65, 74, 13, 19, 60, 38, 23, 53, 5, 6, 12, 73, 59, 51, 30, 58, 80, 24, 0, 9, 42, 64, 52, 41, 61, 21, 31, 27, 17, 67, 33, 62, 4, 11, 63, 48, 10, 70, 34, 2, 44, 45, 46, 1, 29, 15, 26, 16, 7, 56, 71, 35, 40, 28, 37, 76, 25, 43, 79, 54, 49, 14, 50, 72, 36, 18, 55, 75, 3, 8, 47, 22, 32][x]);
//         });
//         solve_single_thread::<DEBUG>(&mut sudoku.clone(), &make_callback_wrapper(&did_solve, callback), &is_cancelled, &|x| x as u8);
//     });
// }

pub fn solve_single_thread<const DEBUG: bool>(sudoku: &mut Sudoku, callback: impl Fn(&Sudoku) -> bool, is_cancelled: impl Fn() -> bool, index_mapper: impl Fn(usize) -> u8) {
    let mut stack: [CandidateSetIterator; 81] = [CandidateSetIterator::empty(); _];
    let mut stack_idx = usize::MAX;
    let mut counter = 0;
    for (k, v) in stack.iter_mut().enumerate() {
        if !sudoku.is_missing(index_mapper(k)) {
            *v = CandidateSetIterator::fixed()
        }
    }

    let pop_task = |sudoku: &mut Sudoku, stack: &mut [CandidateSetIterator; 81], stack_idx: &mut usize, counter: &mut i32| -> bool {
        *stack_idx = (*stack_idx).min(80);
        while *stack_idx <= 80 && (stack[*stack_idx].is_empty() || stack[*stack_idx].is_fixed()) {
            if !stack[*stack_idx].is_fixed() {
                sudoku.set(index_mapper(*stack_idx), 0);
            }
            *stack_idx = stack_idx.wrapping_sub(1);
        }
        if *stack_idx > 80 {
            //Stack is empty, no more tasks
            false
        } else {
            *counter += 1;
            if *counter > 100000 {
                *counter = 0;
                if is_cancelled() {
                    return false;
                }
            }
            sudoku.set(index_mapper(*stack_idx), stack[*stack_idx].next().unwrap());
            if DEBUG {
                println!("Trying:  {}", SingleLineDisplayAdaptor(sudoku));
                println!("         {}^", " ".repeat(index_mapper(*stack_idx) as usize));
            }
            true
        }
    };
    let push_tasks = |sudoku: &mut Sudoku, stack: &mut [CandidateSetIterator; 81], stack_idx: &mut usize| -> bool {
        *stack_idx = (*stack_idx).wrapping_add(1);
        while *stack_idx <= 80 && stack[*stack_idx].is_fixed() {
            *stack_idx += 1;
        }
        if *stack_idx > 80 {
            if callback(&sudoku) {
                return false;
            }
        } else {
            stack[*stack_idx] = sudoku.get_candidates(index_mapper(*stack_idx)).into_iter()
        }
        return true;
    };

    push_tasks(sudoku, &mut stack, &mut stack_idx);
    while pop_task(sudoku, &mut stack, &mut stack_idx, &mut counter) && push_tasks(sudoku, &mut stack, &mut stack_idx) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AnswerChecker<'a> {
        expecting: Option<&'a str>,
    }
    impl<'a> AnswerChecker<'a> {
        fn solution_callback(&self, candidate: &Sudoku) -> bool {
            assert!(candidate.is_valid());
            if self.expecting
        }
    }

    #[test]
    fn test_basic() {
        test_helper_with_answer("351897264897642135642351789265139478784265913139784526918476352523918647476503891","351897264897642135642351789265139478784265913139784526918476352523918647476523891");
        test_helper_with_answer("987645213132978465654123798579364182346812950821597634495236871263781549718459326","987645213132978465654123798579364182346812957821597634495236871263781549718459326");
        test_helper_with_answer("537864129864912375912537648629175483175348296348629751783496512201783964496251837","537864129864912375912537648629175483175348296348629751783496512251783964496251837");
        test_helper_with_answer("275396418481275369396481257813754926962813745754962801627139584139548672548627193","275396418481275369396481257813754926962813745754962831627139584139548672548627193");
        test_helper_with_answer("835791426642853197719624358961248735573916842084537619458379261126485973397162584","835791426642853197719624358961248735573916842284537619458379261126485973397162584");
        test_helper_with_answer("281659734659703218743281695516497823497832561832516479378125946125964387964378152","281659734659743218743281695516497823497832561832516479378125946125964387964378152");
        test_helper_with_answer("849756123756213489213849576532198647198467350467532918625381794974625831381974265","849756123756213489213849576532198647198467352467532918625381794974625831381974265");
        test_helper_with_answer("721645839564893271389217654693178425817452963245936187436789512978521346150364798","721645839564893271389217654693178425817452963245936187436789512978521346152364798");
        test_helper_with_answer("971483526834562179625197438583216704497358261162749385258671943716934852349825617","971483526834562179625197438583216794497358261162749385258671943716934852349825617");
        test_helper_with_answer("971483562526971438403526917219734856734865291865219743147358629358692174692147385","971483562526971438483526917219734856734865291865219743147358629358692174692147385");
        test_helper_with_answer("824517396157936824396284157719362485485179630632845719973628541541793268268451973","824517396157936824396284157719362485485179632632845719973628541541793268268451973");
        test_helper_with_answer("849756231132948756657231948284569317713482569965317482571823694328694170496175823","849756231132948756657231948284569317713482569965317482571823694328694175496175823");
        test_helper_with_answer("215364987879215436364879521531486792927531648486927150752143869143698275698752314","215364987879215436364879521531486792927531648486927153752143869143698275698752314");
        test_helper_with_answer("374612589612958743958374126435267018267891354891435672549723861723186495186549237","374612589612958743958374126435267918267891354891435672549723861723186495186549237");
        test_helper_with_answer("127936548603485127548271693275319486486752931931864275869547312312698754754123869","127936548693485127548271693275319486486752931931864275869547312312698754754123869");
        test_helper_with_answer("374598261621437895985162734198246357537819642462753918219674583746305129853921476","374598261621437895985162734198246357537819642462753918219674583746385129853921476");
        test_helper_with_answer("914623758758194203263578914547319682682457139139862547876945321321786495495231876","914623758758194263263578914547319682682457139139862547876945321321786495495231876");
        test_helper_with_answer("768314295143029768295876143481293657657148932932765481319652874526487319874931526","768314295143529768295876143481293657657148932932765481319652874526487319874931526");
        test_helper_with_answer("914386257386257914257914386172849563560172849849563172698735421735421698421698735","914386257386257914257914386172849563563172849849563172698735421735421698421698735");
        test_helper_with_answer("987643521634215897251078364542891736819367452376524189128739645465182973793456218","987643521634215897251978364542891736819367452376524189128739645465182973793456218");
        test_helper_with_answer("561487392309651874847239516136574928754892163982316745693145287415728639278963451","561487392329651874847239516136574928754892163982316745693145287415728639278963451");
        test_helper_with_answer("519837246264915783738462591187243659342596178695781324823654907971328465456179832","519837246264915783738462591187243659342596178695781324823654917971328465456179832");
        test_helper_with_answer("281673459736549218495128763129867534543912687678354192357491826062735941914286375","281673459736549218495128763129867534543912687678354192357491826862735941914286375");
        test_helper_with_answer("281673495594182673376495182815267349043518267762349518159826734628734951437951826","281673495594182673376495182815267349943518267762349518159826734628734951437951826");
        test_helper_with_answer("721645983938271456465398712142956837596783124873412569254869371310524698689137245","721645983938271456465398712142956837596783124873412569254869371317524698689137245");
        test_helper_with_answer("721645983938217546564389712219476358385192674647853291193720865856931427472568139","721645983938217546564389712219476358385192674647853291193724865856931427472568139");
        test_helper_with_answer("000720030007006820106008709003091000580407200000000006840650010600143900005000402","958724631437916825126538749763291584581467293294385176849652317672143958315879462");
        test_helper_with_answer("900724030030050784100083000093400006001208009000900370016000040304860020200040000","958724631632159784147683592893475216761238459425916378516392847374861925289547163");
        test_helper_with_answer("002700500804020970960800420500078040048000057006430200009000000210007004000040795","132794586854326971967851423521678349348219657796435218479583162215967834683142795");
        test_helper_with_answer("102700580006005790709106042200060400560008210400000000070200000604050003000647008","132794586846325791759186342287961435563478219491532867978213654624859173315647928");
        test_helper_with_answer("009700060100068009000294507000809600800000004030000982900005020605020093007940056","459731268172568349368294517724819635896352174531476982943685721685127493217943856");
        test_helper_with_answer("000080930379500040000073500004300070810090000700406001107609854040700000926000003","265184937379562148481973526654318279813297465792456381137629854548731692926845713");
        test_helper_with_answer("000500341105074600300100000409700100000090004600021970501000706800900425740005000","976582341125374698384169257439756182217893564658421973591248736863917425742635819");
        test_helper_with_answer("307019400800452000000360000040700985730080200080240730620070004000094000090000170","367819452819452367452367819246731985731985246985246731623178594178594623594623178");
        test_helper_with_answer("000009400500207001900534760800000674000490000154000020016050000289601000405002100","367819452548267391921534768893125674672493815154786923716358249289641537435972186");
        test_helper_with_answer("006975213507631849093080675052006000000520086004890150240709000065100008001060004","486975213527631849193284675852416937719523486634897152248759361365142798971368524");
        test_helper_with_answer("003095027900087430700200001108000005009802010352014080830970002507428190090506748","413695827925187436786243951148369275679852314352714689834971562567428193291536748");
        test_helper_with_answer("652070381000680040038100690003800704829040016574260908000008170085019403007420000","652974381791683245438152697163895724829347516574261938946538172285719463317426859");
        test_helper_with_answer("052304000437180920600500400086203107174065302300700008003018250060900014801052030","952374681437186925618529473586293147174865392329741568793418256265937814841652739");
        test_helper_with_answer("241396080000085624805720100004009500176008900090030008020900370610072859059063200","241396785937185624865724193384219567176548932592637418428951376613472859759863241");
        test_helper_with_answer("000074008401005320673102540060040030198023006034569080000007800047018265029450100","952374618481695327673182549765841932198723456234569781516237894347918265829456173");
        test_helper_with_answer("135826000700000520000070001000000034073000215081243076006085740042967153057402609","135826497764139528298574361629751834473698215581243976916385742842967153357412689");
        test_helper_with_answer("570362100064070030931485620020600094603759200095210000306007902009500008782006000","578362149264971835931485627127638594643759281895214376356847912419523768782196453");
        test_helper_with_answer("060314057470650200305028694003000460740580000891402305007040500604801002000900746","269314857478659213315728694523197468746583129891462375937246581654871932182935746");
        test_helper_with_answer("752630914149257683836941275983412567014570398075389421367894152498125736521763849","752638914149257683836941275983412567214576398675389421367894152498125736521763849");
        test_helper_with_answer("248769510967513842315842769659380204183274956472956381794635128536128097821497635","248769513967513842315842769659381274183274956472956381794635128536128497821497635");
        test_helper_with_answer("689124753357608241042075986401539628826417395593862174734956812965281437218743569","689124753357698241142375986471539628826417395593862174734956812965281437218743569");
        test_helper_with_answer("841325967796418325532967418674831502059746831103592746927684153315279604468153279","841325967796418325532967418674831592259746831183592746927684153315279684468153279");
        test_helper_with_answer("801230479497185326362749815613492587508361942924857631239574160186923754745618293","851236479497185326362749815613492587578361942924857631239574168186923754745618293");
        test_helper_with_answer("519823046764519832803764591351482967482976315976350428197035684235648179648197253","519823746764519832823764591351482967482976315976351428197235684235648179648197253");
        test_helper_with_answer("186275934725934816394816075067523491901687523253491687418762359672359048539148762","186275934725934816394816275867523491941687523253491687418762359672359148539148762");
        test_helper_with_answer("275391640406752913139864527528179364643285791917436852792613485061548279850927136","275391648486752913139864527528179364643285791917436852792613485361548279854927136");
        test_helper_with_answer("169284735208375916350196824730951682826743591915862473681427359593618247472039168","169284735248375916357196824734951682826743591915862473681427359593618247472539168");
        test_helper_with_answer("001000000005000000900000200000040000000057000000310402040500630600400805009000000","261794358385621947974835216138246579426957183597318462742589631613472895859163724");
        test_helper_with_answer("009000132060000000000800005100000008000100073000600000201000000500010000900060400","879456132465321789312897645154732968698145273723689514241973856586214397937568421");

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

    #[test]
    fn test_files() {
        use std::fs;
        use std::io::{BufReader, BufRead};
        let paths = fs::read_dir("../test_files").unwrap();
        let mut paths = paths.flatten().collect::<std::vec::Vec<_>>();
        paths.sort_by_key(|x| (x.path().as_os_str().len(), x.path()));
        for path in paths {
            let path = path.path();
            let path_str: String = path.display().to_string();
            print!("Solving file {}...", path_str);
            let test_start = std::time::Instant::now();
            let file = fs::File::open(path).unwrap();
            let mut reader = BufReader::new(file);
            let mut buf = String::with_capacity(81*2+1);

            //Discard first line with headers: PUZZLES,SOLUTIONS,char_count
            _ = reader.read_line(&mut buf);

            loop {
                buf.clear();
                match reader.read_line(&mut buf) {
                    Ok(0) => break,
                    Err(_) => break,
                    //This checks that the solver is able to arrive at the solution specified in the CSV file
                    // _ => test_helper_with_answer(&buf[0..81], &buf[82..163]),
                    //This checks that the solver is able to arrive at _some_ (any) solution
                    _ => test_helper(&buf[0..81]),
                }
            }
            println!("\rFinished solving file {} in {:?}", path_str, std::time::Instant::now()-test_start);
        }
    }
    
    fn test_helper_with_callback<SOLFN: Fn(&Sudoku) -> bool + Send>(solver: &mut Solver<'_, SOLFN>, sudoku_str: &str, callback: SOLFN) {
        let mut sudoku: Sudoku = sudoku_str.into();
        assert!(!sudoku.is_valid());
        solver.solve(&mut sudoku, callback);
    }

    fn prepare_callback_for_any_solution() -> impl Fn(&Sudoku) -> bool {
        use std::sync::atomic::{AtomicBool, Ordering};
        let did_solve = AtomicBool::new(false);
        move |candidate: &Sudoku| {
            assert!(candidate.is_valid());
            did_solve.store(true, Ordering::Release);
            true
        }
    }

    fn solution_callback(candidate: &Sudoku) -> bool {
        use std::sync::atomic::{AtomicBool, Ordering};
        let sudoku_sol: Sudoku = solution.into();
        let did_solve = AtomicBool::new(false);
        assert!(!sudoku.is_valid());
        solver.solve(&mut sudoku, |s: &Sudoku| {
            assert!(s.is_valid());
            if *s == sudoku_sol {
                did_solve.store(true, Ordering::Release);
                true
            } else {
                false
            }
        });
        assert!(did_solve.load(Ordering::Acquire));
    }

    fn make_callback_for_specific_solution() -> impl Fn(&Sudoku) -> bool {
        use std::sync::atomic::{AtomicBool, Ordering};
        let did_solve = AtomicBool::new(false);
        move |candidate: &Sudoku| {
            assert!(candidate.is_valid());
            did_solve.store(true, Ordering::Release);
            true
        }
    }

    fn test_helper(sudoku_str: &str) {
        use std::sync::atomic::{AtomicBool, Ordering};
        let mut sudoku: Sudoku = sudoku_str.into();
        let did_solve = AtomicBool::new(false);
        assert!(!sudoku.is_valid());
        fn callback(candidate: &Sudoku) {
            assert!(s.is_valid());
            did_solve.store(true, Ordering::Release);
            true
        }

        assert!(did_solve.load(Ordering::Acquire));
    }

    fn test_helper_with_answer(&solver: &Solver<'_, impl Fn(&Sudoku) -> bool + Send>,sudoku_str: &str, solution: &str) {
        use std::sync::atomic::{AtomicBool, Ordering};
        let mut sudoku: Sudoku = sudoku_str.into();
        let sudoku_sol: Sudoku = solution.into();
        let did_solve = AtomicBool::new(false);
        assert!(!sudoku.is_valid());
        solver.solve(&mut sudoku, |s: &Sudoku| {
            assert!(s.is_valid());
            if *s == sudoku_sol {
                did_solve.store(true, Ordering::Release);
                true
            } else {
                false
            }
        });
        assert!(did_solve.load(Ordering::Acquire));
    }
}

// Run the solver on all csv files using `cargo test --release -- --no-capture`
fn main() {
    with_multithreaded_solver(|solver| {
        let mut sudoku: Sudoku =
            "000720030007006820106008709003091000580407200000000006840650010600143900005000402".into();
        assert!(!sudoku.is_valid());
        println!("Problem: {}", sudoku);
        solver.solve(&mut sudoku, |s| {
            assert!(s.is_valid());
            println!("Solved:  {}", &s);
            true
        });
    });
}
