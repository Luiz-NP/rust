<div align="center">
  <h1>Programando um jogo de adivinhação</h1>
</div>

### 🦀 Após criar a estrutura básica do projeto, inseri um pouco de código no arquivo ```main.rs```:

  ```rs
  use std::io;

  fn main() {
      println!("Guess the number!");

      println!("Please input your guess.");

      let mut guess = String::new();

      io::stdin()
          .read_line(&mut guess)
          .expect("Failed to read line");

      println!("You guessed: {guess}");
  }
  ```
### 🦀 Um pouco mais complexo que os primeiros exemplos então aqui está uma lista do que há de novo:

- ```rs
    use std::io; // faz a importação do módulo io, que vem da biblioteca padrão std.
  ```
  > Por padrão, rust tem um conjuto de itens definidos na biblioteca padrão, chamado de ```prelúdio```.

- ```rs
    let mut guess = String::new(); // cria uma variável mutável vinculando a ela uma uma nova instancia de String.
  ```
  > Variáveis em rust são imutáveis por padrão. <br/> <br/>
  > String é um tipo fornecido pela biblioteca padrão que é um bit de texto codificado em UTF-8 expansível. <br/> <br/>
  > A sintaxe ```::new``` indica uma função associada ao tipo String, nesse caso, cria uma nova string vazia.

- ```rs
    io::stdin() // chama a função stdin() do modulo io.
    
    .read_line(&mut guess) // método para obter a entrada do usuário.
    
    .expect("Failed to read line"); // mostra uma mensagem caso aja um erro.
  ```
  > O argumento ```&mut guess``` dentro de ```read_line()``` aponta para qual string a função armazenar o valor. <br/> <br/>
  > O ```&``` indica que o argumento é uma referência. Referências, assim como as variáveis, são imutáveis por padrão,
  > então você precisa escrever ```&mut guess``` ao inves de ```&guess```. <br/> <br/>
  > ```read_line``` retorna um ```Result``` que pode variar entre  ```Ok```, que armazena o valor digitado pelo usuário caso tudo corra bem, e ```Err```,
  > que retorna um erro caso aja um.
  > Valores do tipo ```Result```, assim como outros tipos, possuem métodos nele, e um deles é o ```expect```.
  
- Se você não chamar ```expect```, o programa compilará, mas você receberá um aviso:
  <div align="center">
    <img src="https://user-images.githubusercontent.com/101659217/200199972-5a984cb4-6373-4230-bb57-f39cec134b0e.png" />
  </div>
