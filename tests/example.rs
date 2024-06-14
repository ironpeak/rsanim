// use rsanim::prelude::*;

// #[derive(Debug, Clone, PartialEq)]
// pub struct Params {
//     pub speed: f32,
//     pub jump: bool,
// }

// fn create_sm(starting_state: String, params: Params) -> StateMachine<String, Params> {
//     StateMachine::new(
//         starting_state,
//         HashMap::from([
//             (
//                 "idle".to_string(),
//                 State {
//                     duration: 0.5,
//                     repeat: true,
//                 },
//             ),
//             (
//                 "walk".to_string(),
//                 State {
//                     duration: 1.0,
//                     repeat: true,
//                 },
//             ),
//             (
//                 "jump".to_string(),
//                 State {
//                     duration: 0.25,
//                     repeat: false,
//                 },
//             ),
//         ]),
//         vec![
//             Transition {
//                 start_state: TransitionStartState::Node("idle".to_string()),
//                 end_state: TransitionEndState::Node("walk".to_string()),
//                 trigger: TransitionTrigger::Condition(Box::new(|x: &Params| {
//                     x.speed > 0.0 && !x.jump
//                 })),
//             },
//             Transition {
//                 start_state: TransitionStartState::Node("walk".to_string()),
//                 end_state: TransitionEndState::Node("idle".to_string()),
//                 trigger: TransitionTrigger::Condition(Box::new(|x: &Params| {
//                     x.speed <= 0.0 && !x.jump
//                 })),
//             },
//             Transition {
//                 start_state: TransitionStartState::Any,
//                 end_state: TransitionEndState::Node("jump".to_string()),
//                 trigger: TransitionTrigger::Condition(Box::new(|x: &Params| x.jump)),
//             },
//             Transition {
//                 start_state: TransitionStartState::Node("jump".to_string()),
//                 end_state: TransitionEndState::Node("walk".to_string()),
//                 trigger: TransitionTrigger::End,
//             },
//         ],
//         params,
//     )
//     .unwrap()
// }

// fn create_animator(state_machine: StateMachine<String, Params>) -> Animator<String, Params, u8> {
//     Animator::new(
//         state_machine,
//         HashMap::from([
//             (
//                 "idle".to_string(),
//                 vec![
//                     Frame {
//                         progress: 0.00,
//                         value: 0,
//                     },
//                     Frame {
//                         progress: 0.33,
//                         value: 1,
//                     },
//                     Frame {
//                         progress: 0.67,
//                         value: 2,
//                     },
//                 ],
//             ),
//             (
//                 "walk".to_string(),
//                 vec![
//                     Frame {
//                         progress: 0.00,
//                         value: 0,
//                     },
//                     Frame {
//                         progress: 0.33,
//                         value: 1,
//                     },
//                     Frame {
//                         progress: 0.67,
//                         value: 2,
//                     },
//                 ],
//             ),
//             (
//                 "jump".to_string(),
//                 vec![
//                     Frame {
//                         progress: 0.00,
//                         value: 0,
//                     },
//                     Frame {
//                         progress: 0.33,
//                         value: 1,
//                     },
//                     Frame {
//                         progress: 0.67,
//                         value: 2,
//                     },
//                 ],
//             ),
//         ]),
//     )
//     .unwrap()
// }

// #[test]
// fn frame_0_0() {
//     let sm = create_sm(
//         "idle".to_string(),
//         Params {
//             speed: 0.0,
//             jump: false,
//         },
//     );

//     let animator = create_animator(sm);

//     assert_eq!(animator.frame(), &0);
// }

// #[test]
// fn frame_0_25() {
//     let sm = create_sm(
//         "idle".to_string(),
//         Params {
//             speed: 0.0,
//             jump: false,
//         },
//     );

//     let mut animator = create_animator(sm);

//     animator.update(0.125);

//     assert_eq!(animator.frame(), &0);
// }

// #[test]
// fn frame_0_5() {
//     let sm = create_sm(
//         "idle".to_string(),
//         Params {
//             speed: 0.0,
//             jump: false,
//         },
//     );

//     let mut animator = create_animator(sm);

//     animator.update(0.25);

//     assert_eq!(animator.frame(), &1);
// }

// #[test]
// fn frame_0_75() {
//     let sm = create_sm(
//         "idle".to_string(),
//         Params {
//             speed: 0.0,
//             jump: false,
//         },
//     );

//     let mut animator = create_animator(sm);

//     animator.update(0.375);

//     assert_eq!(animator.frame(), &2);
// }

// #[test]
// fn frame_0_99() {
//     let sm = create_sm(
//         "idle".to_string(),
//         Params {
//             speed: 0.0,
//             jump: false,
//         },
//     );

//     let mut animator = create_animator(sm);

//     animator.update(0.495);

//     assert_eq!(animator.frame(), &2);
// }

// #[test]
// fn frame_1_0() {
//     let sm = create_sm(
//         "idle".to_string(),
//         Params {
//             speed: 0.0,
//             jump: false,
//         },
//     );

//     let mut animator = create_animator(sm);

//     animator.update(0.5);

//     assert_eq!(animator.frame(), &0);
// }

// #[test]
// fn starts_in_starting_state() {
//     let sm = create_sm(
//         "idle".to_string(),
//         Params {
//             speed: 0.0,
//             jump: false,
//         },
//     );

//     let animator = create_animator(sm);

//     assert_eq!(
//         animator.state(),
//         &CurrentState {
//             key: "idle".to_string(),
//             duration: 0.5,
//             elapsed: 0.0,
//             repeat: true,
//         }
//     );
// }

// #[test]
// fn idle_repeats() {
//     let sm = create_sm(
//         "idle".to_string(),
//         Params {
//             speed: 0.0,
//             jump: false,
//         },
//     );

//     let mut animator = create_animator(sm);

//     assert_eq!(animator.state().progress(), 0.0);

//     animator.update(0.25);
//     assert_eq!(animator.state().progress(), 0.5);

//     animator.update(0.20);
//     assert_eq!(animator.state().progress(), 0.9);

//     animator.update(0.05);
//     assert_eq!(animator.state().progress(), 0.0);
// }

// #[test]
// fn walk_repeats() {
//     let sm = create_sm(
//         "walk".to_string(),
//         Params {
//             speed: 0.0,
//             jump: false,
//         },
//     );

//     let mut animator = create_animator(sm);

//     assert_eq!(animator.state().progress(), 0.0);

//     animator.update(0.5);
//     assert_eq!(animator.state().progress(), 0.5);

//     animator.update(0.4);
//     assert_eq!(animator.state().progress(), 0.9);

//     animator.update(0.1);
//     assert_eq!(animator.state().progress(), 0.0);
// }

// #[test]
// fn transition_idle_to_walk() {
//     let sm = create_sm(
//         "idle".to_string(),
//         Params {
//             speed: 0.0,
//             jump: false,
//         },
//     );

//     let mut animator = create_animator(sm);

//     animator.update_parameters(&|x| {
//         x.speed = 1.0;
//     });

//     assert_eq!(
//         animator.state(),
//         &CurrentState {
//             key: "walk".to_string(),
//             duration: 1.0,
//             elapsed: 0.0,
//             repeat: true,
//         }
//     );
// }

// #[test]
// fn transition_idle_to_jump() {
//     let sm = create_sm(
//         "idle".to_string(),
//         Params {
//             speed: 0.0,
//             jump: false,
//         },
//     );

//     let mut animator = create_animator(sm);

//     animator.update_parameters(&|x| {
//         x.jump = true;
//     });

//     assert_eq!(
//         animator.state(),
//         &CurrentState {
//             key: "jump".to_string(),
//             duration: 0.25,
//             elapsed: 0.0,
//             repeat: false,
//         }
//     );
// }

// #[test]
// fn transition_walk_to_idle() {
//     let sm = create_sm(
//         "walk".to_string(),
//         Params {
//             speed: 1.0,
//             jump: false,
//         },
//     );

//     let mut animator = create_animator(sm);

//     animator.update_parameters(&|x| {
//         x.speed = 0.0;
//     });

//     assert_eq!(
//         animator.state(),
//         &CurrentState {
//             key: "idle".to_string(),
//             duration: 0.5,
//             elapsed: 0.0,
//             repeat: true,
//         }
//     );
// }

// #[test]
// fn transition_walk_to_jump() {
//     let sm = create_sm(
//         "walk".to_string(),
//         Params {
//             speed: 1.0,
//             jump: false,
//         },
//     );

//     let mut animator = create_animator(sm);

//     animator.update_parameters(&|x| {
//         x.jump = true;
//     });

//     assert_eq!(
//         animator.state(),
//         &CurrentState {
//             key: "jump".to_string(),
//             duration: 0.25,
//             elapsed: 0.0,
//             repeat: false,
//         }
//     );
// }

// #[test]
// fn transition_end_jump_to_idle() {
//     let sm = create_sm(
//         "jump".to_string(),
//         Params {
//             speed: 1.0,
//             jump: true,
//         },
//     );

//     let mut animator = create_animator(sm);

//     animator.update_parameters(&|x| {
//         x.jump = false;
//     });

//     assert_eq!(
//         animator.state(),
//         &CurrentState {
//             key: "jump".to_string(),
//             duration: 0.25,
//             elapsed: 0.0,
//             repeat: false,
//         }
//     );

//     animator.update(0.25);

//     assert_eq!(
//         animator.state(),
//         &CurrentState {
//             key: "walk".to_string(),
//             duration: 1.0,
//             elapsed: 0.0,
//             repeat: true,
//         }
//     );
// }

// #[test]
// fn transition_do_not_transition_to_current_state() {
//     let sm = create_sm(
//         "jump".to_string(),
//         Params {
//             speed: 1.0,
//             jump: true,
//         },
//     );

//     let mut animator = create_animator(sm);

//     animator.update_parameters(&|x| {
//         x.jump = true;
//     });

//     assert_eq!(
//         animator.state(),
//         &CurrentState {
//             key: "jump".to_string(),
//             duration: 0.25,
//             elapsed: 0.0,
//             repeat: false,
//         }
//     );

//     animator.update(0.2);

//     assert_eq!(
//         animator.state(),
//         &CurrentState {
//             key: "jump".to_string(),
//             duration: 0.25,
//             elapsed: 0.2,
//             repeat: false,
//         }
//     );
// }
