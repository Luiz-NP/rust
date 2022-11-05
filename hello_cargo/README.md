<div align="center">
  <h2>Hello Cargo!</h2>
</div>

🦀 Este arquivo executa a mesma função do arquivo ```hello_world```, com algumas pequenas diferenças:

  > Na criação usamos o ```cargo```, que é o gerenciador de pacotes do rust.

  ```js
    $ cargo new filename // cria os aquivos "src" e "Cargo.toml".
  ```
  > O arquivo com a extensão ```.toml```(Tom's Obvious, Minimal Language), é o arquivo de configuração de dependências.
<br/>

🦀 Além de gerenciador de pacotes, o ```cargo``` também atua como compilador para projetos maiores, ou que possuam dependências.

  ```js
    $ cargo build // compila o código
  ```
  > O código compilado vai para uma nova pasta chamada ```target``` junto com alguns arquivos de depuração, depois disso você pode executa-lo com o seguinte:

  ```js
    $ ./target/debug/filename // executa o binário
  ```
<br/>

🦀 Uma alternativa menos trabalhosa para rodar seu código é o ```cargo run```:

  ```js
    $ cargo run // compila e executa
  ```
  
  ### Observações:
   - Criar um projeto usando o ```cargo``` inicia um repósitorio git por padrão, ```caso não esteja em um```.
   - A pasta ```/target``` deve ser ignorada no arquivo ```.gitignore```.
   
