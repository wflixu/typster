import * as monaco from "monaco-editor";
import editorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";
import jsonWorker from "monaco-editor/esm/vs/language/json/json.worker?worker";
import typstConfig from "./lang/typst-config.json";
import bibtex from "./lang/bibtex.json";
import * as oniguruma from "vscode-oniguruma";
import { wireTextMateGrammars } from "./lang/grammar";
import typstTm from "./lang/typst-tm.json";

import { TypstCompletionProvider } from "./lang/completion";
import { Registry } from "vscode-textmate";

type IMonarchLanguage = monaco.languages.IMonarchLanguage;

// @ts-ignore
self.MonacoEnvironment = {
  getWorker(_: any, label: string) {
    if (label === "json") {
      return new jsonWorker();
    }
    return new editorWorker();
  },
};

// Register TextMate grammars
const registry = new Registry({
  onigLib: Promise.resolve(oniguruma),
  // @ts-ignore
  loadGrammar() {
    return Promise.resolve(typstTm);
  },
});

monaco.languages.register({ id: "typst", extensions: ["typ"] });
monaco.languages.setLanguageConfiguration(
  "typst",
  typstConfig as unknown as monaco.languages.LanguageConfiguration
);

// Register Monarch languages
monaco.languages.register({ id: "bibtex", extensions: ["bib"] });
monaco.languages.setMonarchTokensProvider("bibtex", bibtex as IMonarchLanguage);

// Register completion providers
monaco.languages.registerCompletionItemProvider(
  "typst",
  new TypstCompletionProvider()
);

wireTextMateGrammars(registry, { typst: "source.typst" }).then(() => {});
