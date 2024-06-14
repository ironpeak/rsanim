// use rsanim::prelude::*;

// #[derive(Clone, Debug, PartialEq)]
// struct Params {
//     pub speed: f32,
//     pub jump: bool,
// }

// fn create_animator(starting_state: String, params: Params) -> Animator<String, Params, u8> {
//     Animator::<String, Params, u8>::new(
//         StateMachine::new(
//             starting_state,
//             HashMap::from([(
//                 "idle".to_string(),
//                 State {
//                     duration: 0.5,
//                     repeat: true,
//                 },
//             )]),
//             vec![],
//             params,
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
//     .unwrap()
// }

// #[test]
// fn animator_parameters() {
//     let animator = create_animator(
//         "idle".to_string(),
//         Params {
//             speed: 0.0,
//             jump: false,
//         },
//     );

//     assert_eq!(
//         animator.parameters(),
//         &Params {
//             speed: 0.0,
//             jump: false,
//         }
//     );
// }

// #[test]
// fn animator_update_parameters() {
//     let mut animator = create_animator(
//         "idle".to_string(),
//         Params {
//             speed: 0.0,
//             jump: false,
//         },
//     );

//     animator.update_parameters(&|x| {
//         x.speed = 1.0;
//     });

//     assert_eq!(
//         animator.parameters(),
//         &Params {
//             speed: 1.0,
//             jump: false,
//         }
//     );
// }
