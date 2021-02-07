
// Perft_only incompatible feature
#[cfg(all(feature = "perft_only", feature = "default"))]
compile_error!("Feature perft_only is incompatible with default");
#[cfg(all(feature = "perft_only", feature = "openings"))]
compile_error!("Feature perft_only is incompatible with openings");
#[cfg(all(feature = "perft_only", feature = "qsearch"))]
compile_error!("Feature perft_only is incompatible with qsearch");
#[cfg(all(feature = "perft_only", feature = "mctsearch"))]
compile_error!("Feature perft_only is incompatible with mctsearch");
#[cfg(all(feature = "perft_only", feature = "mtdsearch"))]
compile_error!("Feature perft_only is incompatible with mtdsearch");
#[cfg(all(feature = "perft_only", feature = "qsearch"))]
compile_error!("Feature perft_only is incompatible with qsearch");
#[cfg(all(feature = "perft_only", feature = "nnue"))]
compile_error!("Feature perft_only is incompatible with nnue");
#[cfg(all(feature = "perft_only", feature = "static_exchange_eval"))]
compile_error!("Feature perft_only is incompatible with static_exchange_eval");
#[cfg(all(feature = "perft_only", feature = "internal_iterative_deepening"))]
compile_error!("Feature perft_only is incompatible with internal_iterative_deepening");
#[cfg(all(feature = "perft_only", feature = "delta_pruning"))]
compile_error!("Feature perft_only is incompatible with delta_pruning");
#[cfg(all(feature = "perft_only", feature = "futility_pruning"))]
compile_error!("Feature perft_only is incompatible with futility_pruning");
#[cfg(all(feature = "perft_only", feature = "razoring"))]
compile_error!("Feature perft_only is incompatible with razoring");
#[cfg(all(feature = "perft_only", feature = "late_move_reduction"))]
compile_error!("Feature perft_only is incompatible with late_move_reduction");
#[cfg(all(feature = "perft_only", feature = "null_search"))]
compile_error!("Feature perft_only is incompatible with null_search");
#[cfg(all(feature = "perft_only", feature = "history_heuristic"))]
compile_error!("Feature perft_only is incompatible with history_heuristic");
#[cfg(all(feature = "perft_only", feature = "killer_heuristic"))]
compile_error!("Feature perft_only is incompatible with killer_heuristic");
#[cfg(all(feature = "perft_only", feature = "prob_cut"))]
compile_error!("Feature perft_only is incompatible with prob_cut");
#[cfg(all(feature = "perft_only", feature = "clustered_tt"))]
compile_error!("Feature perft_only is incompatible with clustered_tt");

// Delta pruning works in quiescence search
#[cfg(all(feature = "delta_pruning", not(feature = "qsearch")))]
compile_error!("Delta Pruning requires qsearch feature enabled");

// TODO: LMR requires Null window search?