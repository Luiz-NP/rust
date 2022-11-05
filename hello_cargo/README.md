<div align="center">
  <h2>Hello Cargo!</h2>
</div>

### 🦀 Este arquivo executa a mesma função do arquivo ```hello_world```, com algumas pequenas diferenças:

  - Na criação usamos o ```cargo```, que é o gerenciador de pacotes do rust.

  ```js
    $ cargo new filename // cria os aquivos "src" e "Cargo.toml".
  ```
  > O arquivo com a extensão ```.toml```(Tom's Obvious, Minimal Language), são destinados a configuração de dependências no rust.
<br/>

### 🦀 Além de gerenciador de pacotes, o ```cargo``` também atua como compilador para projetos maiores, ou que possuam dependências.

  ```js
    $ cargo build // compila o código
  ```
  > O código compilado vai para ```target/debug/``` junto com alguns arquivos de depuração, depois disso você pode executa-lo com o seguinte:

  ```js
    $ ./target/debug/filename // executa o binário
  ```
<br/>

### 🦀 Uma alternativa menos trabalhosa para rodar seu código é o ```cargo run```:

  ```js
    $ cargo run // compila e executa
  ```
<br/>

### 🦀 O ```cargo``` também oferece um comando que checa rapidamente se seu código para garantir que ele seja compilado:

```js
  $ cargo check // checa o código (sem gerar um binário)
```
  
  ### Observações:
   - Criar um projeto usando o ```cargo``` inicia um repósitorio git por padrão, ```caso não esteja em um```.
   - A pasta ```/target``` deve ser ignorada no arquivo ```.gitignore```.
   - Ao compilar o código usando o ```cargo```, um arquivo ```Cargo.lock``` é gerado, contendo as versões exatas das depêndencias do seu projeto.
   
