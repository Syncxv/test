import { invoke } from "@tauri-apps/api/tauri";

function App() {
	return (
		<div style="display: flex; height: 100vh; align-items:center; justify-content: center; gap: 1rem">
			<button onClick={async () => console.log(await invoke("greet"))}>
				greet
			</button>
			<button onClick={async () => console.log(await invoke("get_config"))}>
				get_config
			</button>
			<button
				onClick={async () =>
					console.log(await invoke("patch", { patchType: "hehe" }))
				}
			>
				patch
			</button>
		</div>
	);
}

export default App;
