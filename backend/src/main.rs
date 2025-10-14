use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <div>
                <h1>"Solana DeFi Portfolio"</h1>
                <p>"Backend is running!"</p>
            </div>
        }
    });
}
