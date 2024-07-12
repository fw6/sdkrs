/* tslint:disable */
/* eslint-disable */
/**
*/
export enum LogLevel {
  Trace = 0,
  Debug = 1,
  Info = 2,
  Warn = 3,
  Error = 4,
}
/**
*/
export class BitwardenClient {
  free(): void;
/**
* @param {string | undefined} [settings_input]
* @param {LogLevel | undefined} [log_level]
*/
  constructor(settings_input?: string, log_level?: LogLevel);
/**
* @param {string} js_input
* @returns {Promise<any>}
*/
  run_command(js_input: string): Promise<any>;
}
