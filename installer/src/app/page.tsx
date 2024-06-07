"use client";
import Image from "next/image";
import React, { useState, useEffect } from "react";
import { RotatingLines } from 'react-loader-spinner';
import { invoke } from '@tauri-apps/api/tauri'
import { emit, listen } from '@tauri-apps/api/event'

export default function Home() {
  const [loadingState, setLoadingState] = useState<string>("Verifying Files");

  useEffect(() => {
    // Initial step: verify file integrity. If this is first launch, then config files should be created. 

    invoke('check_files') // invoke rust code, if fails then the program will auto quit.

    setLoadingState("Installing Glyph. Please be Patient.")

  }, [])

  return (
    <>
    <div className="w-full h-[100vh] bg-white text-black flex items-center justify-center gap-3 flex-col">
      <h1 className="text-5xl font-semibold tracking-tight">Glyph</h1>
      <hr className="mt-8" />
      <p className="text-sm opacity-50">{loadingState}</p>

      <RotatingLines
        visible={true}
        width="20"
        strokeWidth="5"
        animationDuration="0.75"
        ariaLabel="rotating-lines-loading"
        strokeColor="rgba(0, 0, 0, 0.2)"
      />
    </div>

    </>
  );
}
