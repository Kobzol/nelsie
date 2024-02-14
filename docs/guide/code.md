# Syntax highlighting

Nelsie supports syntax highlighting for many common languages ([The list of supported syntaxes](#the-list-of-supported-syntaxes)).

The `.code()` method creates a box with syntax highlighted text. It works similar to the `.text()` method.
except that it takes a name for the syntax highlighter. You can use the name of the language or a filename extension
for the language

```nelsie
@deck.slide()
def code_demo(slide):
    slide.code("""
fn main() {
    println!("Hello world!")
}""", "Rust")
```

## Styling code

You can change the style by passing the parameter `style`:

## Named style `"code"`.

The `.code()` method use the named style `"code"`as default default style. 


## Inline text styles


## Syntax highlighting themes


## The list of supported syntaxes

This list is also programmatically available through [`Resources`](../reference/resources.md)

* ASP (asa)
* ActionScript (as)
* AppleScript (applescript, script editor)
* Batch File (bat, cmd)
* BibTeX (bib)
* Bourne Again Shell (bash) (sh, bash, zsh, fish, .bash_aliases, .bash_completions, .bash_functions, .bash_login, .bash_logout, .bash_profile, .bash_variables, .bashrc, .profile, .textmate_init)
* C (c, h)
* C# (cs, csx)
* C++ (cpp, cc, cp, cxx, c++, C, h, hh, hpp, hxx, h++, inl, ipp)
* CSS (css, css.erb, css.liquid)
* Cargo Build Results ()
* Clojure (clj)
* D (d, di)
* Diff (diff, patch)
* Erlang (erl, hrl, Emakefile, emakefile)
* Go (go)
* Graphviz (DOT) (dot, DOT, gv)
* Groovy (groovy, gvy, gradle)
* HTML (html, htm, shtml, xhtml, inc, tmpl, tpl)
* HTML (ASP) (asp)
* HTML (Erlang) (yaws)
* HTML (Rails) (rails, rhtml, erb, html.erb)
* HTML (Tcl) (adp)
* Haskell (hs)
* JSON (json, sublime-settings, sublime-menu, sublime-keymap, sublime-mousemap, sublime-theme, sublime-build, sublime-project, sublime-completions, sublime-commands, sublime-macro, sublime-color-scheme)
* Java (java, bsh)
* Java Properties (properties)
* Java Server Page (JSP) (jsp)
* JavaDoc ()
* JavaScript (js, htc)
* JavaScript (Rails) (js.erb)
* LaTeX (tex, ltx)
* LaTeX Log ()
* Lisp (lisp, cl, clisp, l, mud, el, scm, ss, lsp, fasl)
* Literate Haskell (lhs)
* Lua (lua)
* MATLAB (matlab)
* Make Output ()
* Makefile (make, GNUmakefile, makefile, Makefile, OCamlMakefile, mak, mk)
* Markdown (md, mdown, markdown, markdn)
* MultiMarkdown ()
* NAnt Build File (build)
* OCaml (ml, mli)
* OCamllex (mll)
* OCamlyacc (mly)
* Objective-C (m, h)
* Objective-C++ (mm, M, h)
* PHP (php, php3, php4, php5, php7, phps, phpt, phtml)
* PHP Source ()
* Pascal (pas, p, dpr)
* Perl (pl, pm, pod, t, PL)
* Plain Text (txt)
* Python (py, py3, pyw, pyi, pyx, pyx.in, pxd, pxd.in, pxi, pxi.in, rpy, cpy, SConstruct, Sconstruct, sconstruct, SConscript, gyp, gypi, Snakefile, wscript)
* R (R, r, s, S, Rprofile)
* R Console ()
* Rd (R Documentation) (rd)
* Regular Expression (re)
* Regular Expressions (Javascript) ()
* Regular Expressions (Python) ()
* Ruby (rb, Appfile, Appraisals, Berksfile, Brewfile, capfile, cgi, Cheffile, config.ru, Deliverfile, Fastfile, fcgi, Gemfile, gemspec, Guardfile, irbrc, jbuilder, podspec, prawn, rabl, rake, Rakefile, Rantfile, rbx, rjs, ruby.rail, Scanfile, simplecov, Snapfile, thor, Thorfile, Vagrantfile)
* Ruby Haml (haml, sass)
* Ruby on Rails (rxml, builder)
* Rust (rs)
* SQL (sql, ddl, dml)
* SQL (Rails) (erbsql, sql.erb)
* Scala (scala, sbt)
* Shell-Unix-Generic ()
* Tcl (tcl)
* TeX (sty, cls)
* Textile (textile)
* XML (xml, xsd, xslt, tld, dtml, rss, opml, svg)
* YAML (yaml, yml, sublime-syntax)
* camlp4 ()
* commands-builtin-shell-bash ()
* reStructuredText (rst, rest)