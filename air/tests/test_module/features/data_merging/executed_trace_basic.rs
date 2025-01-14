/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use air::ExecutionCidState;
use air_interpreter_data::ExecutionTrace;
use air_test_utils::prelude::*;

use pretty_assertions::assert_eq;

#[test]
fn executed_trace_seq_par_call() {
    let local_peer_id = "local_peer_id";
    let remote_peer_id = "remote_peer_id";
    let mut vm = create_avm(unit_call_service(), local_peer_id);

    let script = f!(r#"
        (seq
            (par
                (call "{local_peer_id}" ("local_service_id" "local_fn_name") [] result_1)
                (call "{remote_peer_id}" ("service_id" "fn_name") [] g)
            )
            (call "{local_peer_id}" ("local_service_id" "local_fn_name") [] result_2)
        )"#);

    let mut cid_state = ExecutionCidState::new();
    let unit_call_service_result = "result from unit_call_service";
    let initial_trace = vec![
        par(1, 1),
        scalar_tracked!(
            unit_call_service_result,
            cid_state,
            peer = local_peer_id,
            service = "local_service_id",
            function = "local_fn_name"
        ),
        scalar_tracked!(
            unit_call_service_result,
            cid_state,
            peer = remote_peer_id,
            service = "service_id",
            function = "fn_name"
        ),
    ];
    let initial_data = raw_data_from_trace(initial_trace, cid_state);

    let result = checked_call_vm!(vm, <_>::default(), script, "", initial_data);
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        par(1, 1),
        scalar!(
            unit_call_service_result,
            peer = local_peer_id,
            service = "local_service_id",
            function = "local_fn_name"
        ),
        scalar!(
            unit_call_service_result,
            peer = remote_peer_id,
            service = "service_id",
            function = "fn_name"
        ),
        scalar!(
            unit_call_service_result,
            peer = local_peer_id,
            service = "local_service_id",
            function = "local_fn_name"
        ),
    ];

    assert_eq!(actual_trace, expected_trace);
    assert!(result.next_peer_pks.is_empty());
}

#[test]
fn executed_trace_par_par_call() {
    let local_peer_id = "local_peer_id";
    let remote_peer_id = "remote_peer_id";
    let mut vm = create_avm(unit_call_service(), local_peer_id);

    let script = f!(r#"
        (par
            (par
                (call "{local_peer_id}" ("local_service_id" "local_fn_name") [] result_1)
                (call "{remote_peer_id}" ("service_id" "fn_name") [] g)
            )
            (call "{local_peer_id}" ("local_service_id" "local_fn_name") [] result_2)
        )"#);

    let unit_call_service_result = "result from unit_call_service";
    let mut cid_state = ExecutionCidState::new();
    let initial_state = vec![
        par(2, 1),
        par(1, 0),
        request_sent_by("peer_id_1"),
        scalar_tracked!(
            unit_call_service_result,
            cid_state,
            peer = local_peer_id,
            service = "local_service_id",
            function = "local_fn_name"
        ),
    ];

    let initial_data = raw_data_from_trace(initial_state, cid_state);

    let result = checked_call_vm!(vm, <_>::default(), &script, "", initial_data);
    let actual_trace = trace_from_result(&result);

    let expected_trace = ExecutionTrace::from(vec![
        par(3, 1),
        par(1, 1),
        scalar!(
            unit_call_service_result,
            peer = local_peer_id,
            service = "local_service_id",
            function = "local_fn_name"
        ),
        request_sent_by(local_peer_id),
        scalar!(
            unit_call_service_result,
            peer = local_peer_id,
            service = "local_service_id",
            function = "local_fn_name"
        ),
    ]);

    assert_eq!(actual_trace, expected_trace);
    assert_eq!(result.next_peer_pks, vec![String::from("remote_peer_id")]);

    let initial_state = vec![
        par(3, 0),
        par(1, 1),
        request_sent_by("peer_id_1"),
        request_sent_by(local_peer_id),
    ];

    let initial_data = raw_data_from_trace(initial_state, <_>::default());

    let result = checked_call_vm!(vm, <_>::default(), script, "", initial_data);
    let actual_trace = trace_from_result(&result);

    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn executed_trace_seq_seq() {
    let peer_id_1 = "12D3KooWHk9BjDQBUqnavciRPhAYFvqKBe4ZiPPvde7vDaqgn5er";
    let peer_id_2 = "12D3KooWAzJcYitiZrerycVB4Wryrx22CFKdDGx7c4u31PFdfTbR";
    let mut vm1 = create_avm(unit_call_service(), peer_id_1.clone());
    let mut vm2 = create_avm(unit_call_service(), peer_id_2.clone());

    let script = f!(r#"
        (seq
            (call "{peer_id_1}" ("identity" "") [] void0)
            (seq
                (call "{peer_id_1}" ("add_blueprint" "") [] blueprint_id)
                (call "{peer_id_2}" ("addBlueprint-14d8488e-d10d-474d-96b2-878f6a7d74c8" "") [] void1)
            )
        )
        "#);

    let result = checked_call_vm!(vm2, <_>::default(), &script, "", "");
    assert_eq!(result.next_peer_pks, vec![peer_id_1]);

    let result = checked_call_vm!(vm1, <_>::default(), &script, "", result.data);
    assert_eq!(result.next_peer_pks, vec![peer_id_2]);

    let result = checked_call_vm!(vm2, <_>::default(), script, "", result.data);

    let actual_trace = trace_from_result(&result);

    let call_service_result = "result from unit_call_service";
    let expected_trace = vec![
        scalar!(call_service_result, peer = peer_id_1, service = "identity"),
        scalar!(call_service_result, peer = peer_id_1, service = "add_blueprint"),
        scalar!(
            call_service_result,
            peer = peer_id_2,
            service = "addBlueprint-14d8488e-d10d-474d-96b2-878f6a7d74c8"
        ),
    ];

    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn executed_trace_create_service() {
    let module = "greeting";
    let module_config = json!(
        {
            "name": module,
            "mem_pages_count": 100,
            "logger_enabled": true,
            "wasi": {
                "envs": json!({}),
                "preopened_files": vec!["/tmp"],
                "mapped_dirs": json!({}),
            }
        }
    );

    let module_bytes = json!([1, 2]);
    let blueprint = json!({"name": "blueprint", "dependencies": [module]});

    let add_module_response = "add_module response";
    let add_blueprint_response = "add_blueprint response";
    let create_response = "create response";

    let call_service: CallServiceClosure = Box::new(move |params| -> CallServiceResult {
        let response = match params.service_id.as_str() {
            "add_module" => add_module_response,
            "add_blueprint" => add_blueprint_response,
            "create" => create_response,
            _ => "unknown response",
        };
        CallServiceResult::ok(json!(response))
    });

    let init_peer_id = "A";
    let set_variables_id = "set_variables";
    let mut vm = create_avm(call_service, init_peer_id);

    let script = include_str!("./scripts/create_service.air");

    let mut cid_state = ExecutionCidState::new();

    let add_module_response = "add_module response";
    let add_blueprint_response = "add_blueprint response";
    let create_response = "create response";
    let expected_trace = vec![
        scalar_tracked!(
            (module_bytes.clone()),
            cid_state,
            peer = set_variables_id,
            service = "add_module",
            args = vec!["module_bytes"]
        ),
        scalar_tracked!(
            (module_config.clone()),
            cid_state,
            peer = set_variables_id,
            service = "add_module",
            args = vec!["module_config"]
        ),
        scalar_tracked!(
            (blueprint.clone()),
            cid_state,
            peer = set_variables_id,
            service = "add_module",
            args = vec!["blueprint"]
        ),
        scalar_tracked!(
            add_module_response,
            cid_state,
            peer = init_peer_id,
            service = "add_module",
            args = vec![module_bytes, module_config]
        ),
        scalar_tracked!(
            add_blueprint_response,
            cid_state,
            peer = init_peer_id,
            service = "add_blueprint",
            args = vec![blueprint]
        ),
        scalar_tracked!(
            create_response,
            cid_state,
            peer = init_peer_id,
            service = "create",
            args = vec![add_blueprint_response]
        ),
        scalar_tracked!("test", cid_state, peer = "remote_peer_id", args = vec![create_response]),
    ];
    let initial_data = raw_data_from_trace(expected_trace.clone(), cid_state);

    let result = checked_call_vm!(vm, <_>::default(), script, "", initial_data);

    let actual_trace = trace_from_result(&result);

    assert_eq!(actual_trace, expected_trace);
    assert!(result.next_peer_pks.is_empty());
}

#[test]
fn executed_trace_par_seq_fold_call() {
    let return_numbers_call_service: CallServiceClosure = Box::new(|_| -> CallServiceResult {
        CallServiceResult::ok(json!(["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]))
    });

    let mut vm1 = create_avm(return_numbers_call_service, "some_peer_id_1");
    let mut vm2 = create_avm(echo_call_service(), "some_peer_id_2");
    let mut vm3 = create_avm(unit_call_service(), "some_peer_id_3");

    let script = r#"
        (par
            (seq
                (call "some_peer_id_1" ("local_service_id" "local_fn_name") [] IterableResultPeer1)
                (fold IterableResultPeer1 i
                    (par
                        (call "some_peer_id_2" ("local_service_id" "local_fn_name") [i] $acc)
                        (next i)
                    )
                )
            )
            (call "some_peer_id_3" ("local_service_id" "local_fn_name") [] result_2)
        )"#;

    let result = checked_call_vm!(vm2, <_>::default(), script, "", "");
    let result = checked_call_vm!(vm1, <_>::default(), script, "", result.data);
    let mut data = result.data;

    for _ in 0..100 {
        let result = checked_call_vm!(vm2, <_>::default(), script, "", data);
        data = result.data;
    }

    let result = checked_call_vm!(vm3, <_>::default(), script, "", data);
    let actual_trace = trace_from_result(&result);

    let generation = 0;
    let expected_trace = ExecutionTrace::from(vec![
        par(21, 1),
        scalar!(
            json!(["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]),
            peer = "some_peer_id_1",
            service = "local_service_id",
            function = "local_fn_name"
        ),
        par(1, 18),
        stream!(
            1.to_string(),
            generation,
            peer = "some_peer_id_2",
            service = "local_service_id",
            function = "local_fn_name",
            args = vec!["1"]
        ),
        par(1, 16),
        stream!(
            2.to_string(),
            generation,
            peer = "some_peer_id_2",
            service = "local_service_id",
            function = "local_fn_name",
            args = vec!["2"]
        ),
        par(1, 14),
        stream!(
            3.to_string(),
            generation,
            peer = "some_peer_id_2",
            service = "local_service_id",
            function = "local_fn_name",
            args = vec!["3"]
        ),
        par(1, 12),
        stream!(
            4.to_string(),
            generation,
            peer = "some_peer_id_2",
            service = "local_service_id",
            function = "local_fn_name",
            args = vec!["4"]
        ),
        par(1, 10),
        stream!(
            5.to_string(),
            generation,
            peer = "some_peer_id_2",
            service = "local_service_id",
            function = "local_fn_name",
            args = vec!["5"]
        ),
        par(1, 8),
        stream!(
            6.to_string(),
            generation,
            peer = "some_peer_id_2",
            service = "local_service_id",
            function = "local_fn_name",
            args = vec!["6"]
        ),
        par(1, 6),
        stream!(
            7.to_string(),
            generation,
            peer = "some_peer_id_2",
            service = "local_service_id",
            function = "local_fn_name",
            args = vec!["7"]
        ),
        par(1, 4),
        stream!(
            8.to_string(),
            generation,
            peer = "some_peer_id_2",
            service = "local_service_id",
            function = "local_fn_name",
            args = vec!["8"]
        ),
        par(1, 2),
        stream!(
            9.to_string(),
            generation,
            peer = "some_peer_id_2",
            service = "local_service_id",
            function = "local_fn_name",
            args = vec!["9"]
        ),
        par(1, 0),
        stream!(
            10.to_string(),
            generation,
            peer = "some_peer_id_2",
            service = "local_service_id",
            function = "local_fn_name",
            args = vec!["10"]
        ),
        scalar!(
            "result from unit_call_service",
            peer = "some_peer_id_3",
            service = "local_service_id",
            function = "local_fn_name"
        ),
    ]);

    assert_eq!(actual_trace, expected_trace);
    assert!(result.next_peer_pks.is_empty());
}

#[test]
fn executed_trace_par_seq_fold_in_cycle_call() {
    let return_numbers_call_service: CallServiceClosure = Box::new(|_| -> CallServiceResult {
        CallServiceResult::ok(json!(["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]))
    });

    let mut vm1 = create_avm(return_numbers_call_service, "some_peer_id_1");
    let mut vm2 = create_avm(echo_call_service(), "some_peer_id_2");
    let mut vm3 = create_avm(unit_call_service(), "some_peer_id_3");

    let script = r#"
        (par 
            (seq 
                (call "some_peer_id_1" ("local_service_id" "local_fn_name") [] IterableResultPeer1)
                (fold IterableResultPeer1 i
                    (par 
                        (call "some_peer_id_2" ("local_service_id" "local_fn_name") [i] $acc)
                        (next i)
                    )
                )
            )
            (call "some_peer_id_3" ("local_service_id" "local_fn_name") [] result_2)
        )"#;

    let mut data = vec![];

    for _ in 0..100 {
        let result = checked_call_vm!(vm1, <_>::default(), script, "", data);
        let result = checked_call_vm!(vm2, <_>::default(), script, "", result.data);
        let result = checked_call_vm!(vm3, <_>::default(), script, "", result.data);

        let actual_trace = trace_from_result(&result);

        let generation = 0;
        let expected_trace = vec![
            par(21, 1),
            scalar!(
                json!(["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]),
                peer = "some_peer_id_1",
                service = "local_service_id",
                function = "local_fn_name"
            ),
            par(1, 18),
            stream!(
                1.to_string(),
                generation,
                peer = "some_peer_id_2",
                service = "local_service_id",
                function = "local_fn_name",
                args = vec!["1"]
            ),
            par(1, 16),
            stream!(
                2.to_string(),
                generation,
                peer = "some_peer_id_2",
                service = "local_service_id",
                function = "local_fn_name",
                args = vec!["2"]
            ),
            par(1, 14),
            stream!(
                3.to_string(),
                generation,
                peer = "some_peer_id_2",
                service = "local_service_id",
                function = "local_fn_name",
                args = vec!["3"]
            ),
            par(1, 12),
            stream!(
                4.to_string(),
                generation,
                peer = "some_peer_id_2",
                service = "local_service_id",
                function = "local_fn_name",
                args = vec!["4"]
            ),
            par(1, 10),
            stream!(
                5.to_string(),
                generation,
                peer = "some_peer_id_2",
                service = "local_service_id",
                function = "local_fn_name",
                args = vec!["5"]
            ),
            par(1, 8),
            stream!(
                6.to_string(),
                generation,
                peer = "some_peer_id_2",
                service = "local_service_id",
                function = "local_fn_name",
                args = vec!["6"]
            ),
            par(1, 6),
            stream!(
                7.to_string(),
                generation,
                peer = "some_peer_id_2",
                service = "local_service_id",
                function = "local_fn_name",
                args = vec!["7"]
            ),
            par(1, 4),
            stream!(
                8.to_string(),
                generation,
                peer = "some_peer_id_2",
                service = "local_service_id",
                function = "local_fn_name",
                args = vec!["8"]
            ),
            par(1, 2),
            stream!(
                9.to_string(),
                generation,
                peer = "some_peer_id_2",
                service = "local_service_id",
                function = "local_fn_name",
                args = vec!["9"]
            ),
            par(1, 0),
            stream!(
                10.to_string(),
                generation,
                peer = "some_peer_id_2",
                service = "local_service_id",
                function = "local_fn_name",
                args = vec!["10"]
            ),
            scalar!(
                "result from unit_call_service",
                peer = "some_peer_id_3",
                service = "local_service_id",
                function = "local_fn_name"
            ),
        ];

        assert_eq!(actual_trace, expected_trace);

        data = result.data;
    }
}

#[test]
fn executed_trace_seq_par_seq_seq() {
    let peer_id_1 = "12D3KooWHk9BjDQBUqnavciRPhAYFvqKBe4ZiPPvde7vDaqgn5er";
    let peer_id_2 = "12D3KooWAzJcYitiZrerycVB4Wryrx22CFKdDGx7c4u31PFdfTbR";
    let mut vm1 = create_avm(unit_call_service(), peer_id_1);
    let mut vm2 = create_avm(unit_call_service(), peer_id_2);
    let script = f!(r#"
        (seq 
            (par 
                (seq 
                    (call "{peer_id_1}" ("" "") [] result_1)
                    (call "{peer_id_2}" ("" "") [] result_2)
                )
                (seq 
                    (call "{peer_id_2}" ("" "") [] result_3)
                    (call "{peer_id_1}" ("" "") [] result_4)
                )
            )
            (call "{peer_id_2}" ("" "") [] result_5)
        )
        "#);

    let result = checked_call_vm!(vm2, <_>::default(), &script, "", "");
    assert_eq!(result.next_peer_pks, vec![peer_id_1.to_string()]);

    let result = checked_call_vm!(vm1, <_>::default(), &script, "", result.data);
    assert_eq!(result.next_peer_pks, vec![peer_id_2.to_string()]);

    let result = checked_call_vm!(vm2, <_>::default(), script, "", result.data);

    let actual_trace = trace_from_result(&result);

    let unit_call_service_result = "result from unit_call_service";
    let executed_trace = vec![
        par(2, 2),
        scalar!(unit_call_service_result, peer = peer_id_1),
        scalar!(unit_call_service_result, peer = peer_id_2),
        scalar!(unit_call_service_result, peer = peer_id_2),
        scalar!(unit_call_service_result, peer = peer_id_1),
        scalar!(unit_call_service_result, peer = peer_id_2),
    ];

    assert_eq!(actual_trace, executed_trace);
    assert!(result.next_peer_pks.is_empty());
}
