import * as monaco from "monaco-editor";

import typstConfig from "./lang/typst-config.json";
import bibtex from "./lang/bibtex.json";
import * as oniguruma from "vscode-oniguruma";
import { wireTextMateGrammars } from "./lang/grammar";
import typstTm from "./lang/typst-tm.json";
import onigurumaWasm from "vscode-oniguruma/release/onig.wasm?url";

import { TypstCompletionProvider } from "./lang/completion";
import { Registry } from "vscode-textmate";
import editorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";
import jsonWorker from "monaco-editor/esm/vs/language/json/json.worker?worker";

type IMonarchLanguage = monaco.languages.IMonarchLanguage;

self.MonacoEnvironment = {
  getWorker(_: any, label: string) {
    if (label === "json") {
      return new jsonWorker();
    }
    return new editorWorker();
  },
};

const useInitMonaco = async () => {
  await fetch(onigurumaWasm)
    .then((res) => res.arrayBuffer())
    .then((wasm) => {
      return oniguruma.loadWASM(wasm);
    });

  // Register TextMate grammars
  const registry = new Registry({
    onigLib: Promise.resolve(oniguruma),
    // @ts-ignore
    loadGrammar() {
      return Promise.resolve(typstTm);
    },
  });

  const grammars = new Map();
  grammars.set("typst", "source.typst");

  monaco.languages.register({ id: "typst", extensions: ["typ"] });
  monaco.languages.setLanguageConfiguration(
    "typst",
    typstConfig as unknown as monaco.languages.LanguageConfiguration
  );

  await wireTextMateGrammars(registry, { typst: "source.typst" }).then(
    () => {}
  );

  // Register Monarch languages
  monaco.languages.register({ id: "bibtex", extensions: ["bib"] });
  monaco.languages.setMonarchTokensProvider(
    "bibtex",
    bibtex as IMonarchLanguage
  );

  // Register completion providers
  monaco.languages.registerCompletionItemProvider(
    "typst",
    new TypstCompletionProvider()
  );
};

useInitMonaco()
  .then((res) => {
    console.log(res);
  })
  .catch((err) => console.log(err));
