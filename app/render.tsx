'use client'

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri'
import React from 'react';

export function SetTextDisplay() {
    const [text, setText] = useState('');
    
    useEffect(() => {
        invoke<string>('get_current_text')
            .then(result => setText(result.split(' ')[0]))
            .catch(console.error)
        }, [])

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
        <textarea onChange={(event) => console.log(event.target.value)} defaultValue={text} />
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
        <input onChange={(event) => console.log(event.target.value)} type="number" defaultValue={speed} />
    )
}
