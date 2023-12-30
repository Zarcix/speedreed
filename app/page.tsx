'use client';

import React from 'react';
import { SetTextDisplay, SetInputDisplay, SetReadingSpeed } from './render'

export default function Home() {
  return (
    <main>
      <CreateTextDisplay />

      <br/>
      <hr/>
      <br/>

      <CreateInput />
    </main>
  )
}

function CreateTextDisplay() {
  return (
    <div className="Center Vertical">
        <div>
          <h1>Read Below!</h1>
        </div>
        <div className="Content">
          <SetTextDisplay />
        </div>
      </div>
  )
}

function CreateInput() {
  return (
    <div className="Center Vertical">
      <label>Input Text</label>
      <SetInputDisplay />

      <br/>

      <label>Reading Speed</label>
      <SetReadingSpeed />
    </div>
  )
}
