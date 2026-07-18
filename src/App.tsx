import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useDiscoveryBackgroundService } from "./hooks/useDiscovery";




function App() {
  const state = useDiscoveryBackgroundService()

  return (
    <main className="container">

      <div>
        {
          state && state != null && state.code === 200 && state.message && state.message !== null && (
              <div>
                {JSON.stringify(state, null, 2) }
              </div>
          )
        }
  
      </div>
{/* 
      <button onClick={async () => {
        const data = await discoveryBackgroundService()
        console.log("discoveryBackgroundService", data)
      }}>
        cl
      </button> */}
    </main>
  )
}

export default App;

// import "./App.css";
// import { usePdfToJpegStore } from "./store/pdfToJpegStore";
// import { getJobId, JobEvent, UIAction } from "./types/JobEvent";


// const JOB_EVENT_NAME = "job-event";


// function App() {
//   const [greetMsg, setGreetMsg] = useState("");
//   const [name, setName] = useState("");
  
//   const jobs = usePdfToJpegStore((s) => s.jobs);

//   async function greet() {
//     // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//     setGreetMsg(await invoke("greet", { name }));
//   }


//   useEffect(() => {
//     const unlisten = listen<JobEvent>(JOB_EVENT_NAME, (event) => {
//       console.log(JOB_EVENT_NAME, event);
//       usePdfToJpegStore.getState().setJob(event.payload);
//     });

//     return () => {
//       unlisten.then((f) => f());
//     };
//   }, []);

//   return (
//     <main className="container">

//       <h1>Job PDF To JPEG</h1>
      
      
//       {
//         Object.values(jobs).map((job) => (
//                 <div key={getJobId(job)}>
//                   {
//                     JSON.stringify(job, null, 2)
//                   }
//                   <p>{job.cmd_name} - {job.resource_path} @ {job.created_at}                   
//                     {
//                         job.ui_action === UIAction.PasswordRequired && (
//                           <span className="text-red-500">Password required for {job.resource_path}</span>
//                         )
//                   }</p>
//                 </div>
//         ))
              
//       }
//     </main>
//     // <main className="container">
//     //   <h1 className="text-red-500">Welcome to Tauri</h1>

//     //   <div className="row">
//     //     <a href="https://vitejs.dev" target="_blank">
//     //       <img src="/vite.svg" className="logo vite" alt="Vite logo" />
//     //     </a>
//     //     <a href="https://tauri.app" target="_blank">
//     //       <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
//     //     </a>
//     //     <a href="https://reactjs.org" target="_blank">
//     //       <img src={reactLogo} className="logo react" alt="React logo" />
//     //     </a>
//     //   </div>
//     //   <p>Click on the Tauri, Vite, and React logos to learn more.</p>

//     //   <form
//     //     className="row"
//     //     onSubmit={(e) => {
//     //       e.preventDefault();
//     //       greet();
//     //     }}
//     //   >
//     //     <input

//     //       id="greet-input"
//     //       onChange={(e) => setName(e.currentTarget.value)}
//     //       placeholder="Enter a name..."
//     //     />
//     //     <button type="submit">Greet</button>
//     //   </form>
//     //   <p>{greetMsg}</p>
//     // </main>
//   );
// }

// export default App;
