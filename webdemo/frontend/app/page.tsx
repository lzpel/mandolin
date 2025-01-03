"use client";
import React from "react";
import {sums, upper} from "@/lib";

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
	// Without async/await
	console.log(sums(100))
	console.log(upper("abc"))
	console.log(upper(openapi))
	return (
		<>
			<textarea id="a"/>
			<button id="addButton">generate</button>
			<span id="result">output</span>
		</>
	)
}
