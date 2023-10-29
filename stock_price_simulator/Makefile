run_debug:
	RUST_BACKTRACE=1 cargo run -- \
        --n-paths 20 \
        --n-steps 100 \
        --sigma 0.4 \
        --r 0.05 \
        --maturity 1.0 \
        --initial-stock-price 100.0 \
        --output-path outputs/simulation.csv

plot:
	Rscript viz.R && rm Rplots.pdf

debug:
	rust-gdb target/debug/stock_price_simulator
