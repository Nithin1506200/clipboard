
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

/** user-defined commands **/


export const commands = {
async health(slug: string) : Promise<string> {
    return await TAURI_INVOKE("health", { slug });
},
async getAllId(offset: number, limit: number) : Promise<string[]> {
    return await TAURI_INVOKE("get_all_id", { offset, limit });
},
async getById(id: string) : Promise<Data | null> {
    return await TAURI_INVOKE("get_by_id", { id });
},
async getAllData(offset: number, limit: number) : Promise<AllData[]> {
    return await TAURI_INVOKE("get_all_data", { offset, limit });
},
async deleteById(id: string) : Promise<Result<null, null>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("delete_by_id", { id }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async updateDataById(id: string, newData: Data) : Promise<Result<null, null>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("update_data_by_id", { id, newData }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async fuzzySearch(query: string) : Promise<Result<Data[], string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("fuzzy_search", { query }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
}
}

/** user-defined events **/



/** user-defined constants **/



/** user-defined types **/

export type AllData = { id: string; data: Data }
export type Data = { tag: "Email"; content: string } | { tag: "PhoneNumber"; content: string } | { tag: "JsonDict"; content: unknown } | { tag: "Code"; content: { data: string; lang: string } }

/** tauri-specta globals **/

import {
	invoke as TAURI_INVOKE,
	Channel as TAURI_CHANNEL,
} from "@tauri-apps/api/core";
import * as TAURI_API_EVENT from "@tauri-apps/api/event";
import { type WebviewWindow as __WebviewWindow__ } from "@tauri-apps/api/webviewWindow";

type __EventObj__<T> = {
	listen: (
		cb: TAURI_API_EVENT.EventCallback<T>,
	) => ReturnType<typeof TAURI_API_EVENT.listen<T>>;
	once: (
		cb: TAURI_API_EVENT.EventCallback<T>,
	) => ReturnType<typeof TAURI_API_EVENT.once<T>>;
	emit: null extends T
		? (payload?: T) => ReturnType<typeof TAURI_API_EVENT.emit>
		: (payload: T) => ReturnType<typeof TAURI_API_EVENT.emit>;
};

export type Result<T, E> =
	| { status: "ok"; data: T }
	| { status: "error"; error: E };

function __makeEvents__<T extends Record<string, any>>(
	mappings: Record<keyof T, string>,
) {
	return new Proxy(
		{} as unknown as {
			[K in keyof T]: __EventObj__<T[K]> & {
				(handle: __WebviewWindow__): __EventObj__<T[K]>;
			};
		},
		{
			get: (_, event) => {
				const name = mappings[event as keyof T];

				return new Proxy((() => {}) as any, {
					apply: (_, __, [window]: [__WebviewWindow__]) => ({
						listen: (arg: any) => window.listen(name, arg),
						once: (arg: any) => window.once(name, arg),
						emit: (arg: any) => window.emit(name, arg),
					}),
					get: (_, command: keyof __EventObj__<any>) => {
						switch (command) {
							case "listen":
								return (arg: any) => TAURI_API_EVENT.listen(name, arg);
							case "once":
								return (arg: any) => TAURI_API_EVENT.once(name, arg);
							case "emit":
								return (arg: any) => TAURI_API_EVENT.emit(name, arg);
						}
					},
				});
			},
		},
	);
}
