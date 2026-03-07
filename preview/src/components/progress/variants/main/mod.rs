use dioxus::prelude::*;
use dioxus_primitives::progress::{Progress, ProgressIndicator};

#[component]
pub fn Demo() -> Element {
    let mut progress = use_signal(|| 0);

    use_effect(move || {
        let mut timer = document::eval(
            "setInterval(() => {
                dioxus.send(Math.floor(Math.random() * 30));
            }, 1000);",
        );
        spawn(async move {
            while let Ok(new_progress) = timer.recv::<usize>().await {
                let mut progress = progress.write();
                *progress = (*progress + new_progress) % 101;
            }
        });
    });

    rsx! {
        Progress {
            style: "position: relative; overflow: hidden; width: 200px; height: 0.5rem; border-radius: 9999px; background: var(--muted);",
            aria_label: "Progressbar Demo",
            value: progress() as f64,
            ProgressIndicator {
                style: "height: 100%; background: var(--foreground); transition: width 250ms ease;",
            }
        }
    }
}
