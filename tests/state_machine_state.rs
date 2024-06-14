// use rsanim::prelude::*;

// #[derive(Clone, Eq, PartialEq, Hash, Debug)]
// enum Animation {
//     Idle,
// }

// #[derive(Clone, Debug, PartialEq)]
// struct Params {
//     pub speed: f32,
//     pub jump: bool,
// }

// fn create_sm(starting_state: Animation, params: Params) -> StateMachine<Animation, Params> {
//     StateMachine::new(
//         starting_state,
//         HashMap::from([(
//             Animation::Idle,
//             State {
//                 duration: 0.5,
//                 repeat: true,
//             },
//         )]),
//         vec![],
//         params,
//     )
//     .unwrap()
// }

// #[test]
// fn sm_state() {
//     let sm = create_sm(
//         Animation::Idle,
//         Params {
//             speed: 0.0,
//             jump: false,
//         },
//     );

//     assert_eq!(
//         sm.state(),
//         &CurrentState {
//             key: Animation::Idle,
//             elapsed: 0.0,
//             duration: 0.5,
//             repeat: true,
//         }
//     );
// }

// #[test]
// fn sm_update_state() {
//     let mut sm = create_sm(
//         Animation::Idle,
//         Params {
//             speed: 0.0,
//             jump: false,
//         },
//     );

//     sm.update(0.25);

//     assert_eq!(
//         sm.state(),
//         &CurrentState {
//             key: Animation::Idle,
//             elapsed: 0.25,
//             duration: 0.5,
//             repeat: true,
//         }
//     );
// }
