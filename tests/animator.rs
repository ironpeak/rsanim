// use rsanim::prelude::*;

// #[derive(Debug, Clone, PartialEq)]
// pub struct Params {
//     pub speed: f32,
//     pub jump: bool,
// }

// #[test]
// fn clone() {
//     let animator = Animator::new(
//         StateMachine::new(
//             "idle".to_string(),
//             HashMap::from([(
//                 "idle".to_string(),
//                 State {
//                     duration: 0.5,
//                     repeat: true,
//                 },
//             )]),
//             vec![],
//             Params {
//                 speed: 1.0,
//                 jump: false,
//             },
//         )
//         .unwrap(),
//         HashMap::from([(
//             "idle".to_string(),
//             vec![
//                 Frame {
//                     progress: 0.00,
//                     value: 0,
//                 },
//                 Frame {
//                     progress: 0.33,
//                     value: 1,
//                 },
//                 Frame {
//                     progress: 0.67,
//                     value: 2,
//                 },
//             ],
//         )]),
//     )
//     .unwrap();

//     assert_eq!(format!("{:?}", animator.clone()), format!("{:?}", animator));
// }

// #[test]
// fn debug() {
//     let animator = Animator::new(
//         StateMachine::new(
//             "idle".to_string(),
//             HashMap::from([(
//                 "idle".to_string(),
//                 State {
//                     duration: 0.5,
//                     repeat: true,
//                 },
//             )]),
//             vec![],
//             Params {
//                 speed: 1.0,
//                 jump: false,
//             },
//         )
//         .unwrap(),
//         HashMap::from([(
//             "idle".to_string(),
//             vec![
//                 Frame {
//                     progress: 0.00,
//                     value: 0,
//                 },
//                 Frame {
//                     progress: 0.33,
//                     value: 1,
//                 },
//                 Frame {
//                     progress: 0.67,
//                     value: 2,
//                 },
//             ],
//         )]),
//     )
//     .unwrap();

//     assert_eq!(format!("{:?}", animator), "Animator { state_machine: StateMachine { current_state: CurrentState { key: \"idle\", duration: 0.5, elapsed: 0.0, repeat: true }, states: {\"idle\": State { duration: 0.5, repeat: true }}, transitions: [], parameters: Params { speed: 1.0, jump: false } }, state_frames: {\"idle\": [Frame { progress: 0.0, value: 0 }, Frame { progress: 0.33, value: 1 }, Frame { progress: 0.67, value: 2 }]} }");
// }
