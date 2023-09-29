import { $, component$, useSignal } from '@builder.io/qwik'
import { invoke } from '@tauri-apps/api/tauri'

export default component$(() => {
  const greetMsg = useSignal('')

  const greet = $(async (name: string) => {
    greetMsg.value = await invoke('greet', { name })
  })

  return (
    <div>
      <button onClick$={() => greet('Qwik')}>Greet</button>
      <p>{greetMsg.value}</p>
    </div>
  )
})