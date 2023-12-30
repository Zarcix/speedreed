'use client'

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri'

export function Greet() {
  const [greeting, setGreeting] = useState('');

  useEffect(() => {
    invoke<string>('test', { input: 'Next.js' })
      .then(result => setGreeting(result))
      .catch(console.error)
  }, [])

  // Necessary because we will have to use Greet as a component later.
  return <div>{greeting}</div>;
}

export function ButtonClick() {

    invoke<string>('test', { input: 'newtext' })
        .then((result) => (document.getElementById("future-text") as HTMLElement).innerHTML = result)
        .catch(console.error)
}