'use client'

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'

export function TextUpdate() {
    const [text, setText] = useState('');

    useEffect(() => {
        listen('text-update', (event: any) => {
            console.log("Text Updating: " + event.payload.message)
            setText(event.payload.message)
        })
    })

    return (
        <h1 id="changing-text">{text}</h1>
        )
}

export function SetInputDisplay() {
    const [text, setText] = useState('');
    useEffect(() => {
        invoke<string>('get_current_text')
            .then(result => setText(result))
            .catch(console.error)
        }, [])

    

    return (
        <textarea onChange={(event) => invoke('set_current_text', { text: event.target.value })} defaultValue={text} />
        )
}

export function SetReadingSpeed() {
    const [speed, setSpeed] = useState(0);

    useEffect(() => {
        invoke<number>('get_current_speed')
            .then(speed => setSpeed(speed))
            .catch(console.error)
        }, [])
    
    return (
        <input onChange={(event) => invoke('set_current_speed', { speed: event.target.value })} type="number" defaultValue={speed} />
    )
}
