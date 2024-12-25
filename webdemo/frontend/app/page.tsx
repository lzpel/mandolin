import dynamic from "next/dynamic";
import React from "react";

//const wasm = dynamic(() => import('../../pkg'), {ssr: false});

const openapi = "openapi: 3.0.0\n" +
	"info:\n" +
	"  title: Simple API\n" +
	"  version: 1.0.0\n" +
	"paths:\n" +
	"  /hello:\n" +
	"    get:\n" +
	"      summary: Get a hello message\n" +
	"      responses:\n" +
	"        '200':\n" +
	"          description: A successful response\n" +
	"          content:\n" +
	"            application/json:\n" +
	"              schema:\n" +
	"                type: object\n" +
	"                properties:\n" +
	"                  message:\n" +
	"                    type: string\n" +
	"                    example: Hello, world!\n"
export default function Home() {
	return (
		<>
			<textarea id="a" value="openapi"/>
			<button id="addButton" onClick={onClick}>generate</button>
			<span id="result">output</span>
		</>
	)
}

function onClick() {
	const value: string = document.getElementById('input')?.nodeValue ?? "";
	//console.log(wasm.example(value))
}
