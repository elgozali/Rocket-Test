#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::response::content;

#[get("/")]
fn page_index() -> content::Html<&'static str> {
    content::Html(r#"
        <meta charset="utf-8">
		<title>Task 1</title>
		<meta name="description" content="Application written in Rocket.">
		<meta name="author" content="Moe Elgozali">
		<meta name="viewport" content="width=device-width, initial-scale=1">
        <title>Task 1</title>
		<link rel="stylesheet" href="https://stackpath.bootstrapcdn.com/bootstrap/4.1.3/css/bootstrap.min.css" integrity="sha384-MCw98/SFnGE8fJT3GXwEOngsV7Zt27NXFoaoApmYm81iuXoPkFOJwJ8ERdknLPMO" crossorigin="anonymous">
		<style type="text/css">
		    select > option{
		        cursor: pointer;
		    }
		</style>
        <form action="/index" method="post">
            <div id="app" class="container">
		    <h1>Select a value:</h1>
		    <select v-model="selectedNode" @change="selectNode" class="form-control">
		    	<option v-for="(node,index) in nodes" :value="index">{{ node.label }}</option>
		  	</select>
		  <hr/>
		    <h2>Real-time Value</h2>
		    <p v-if="selectedNode != -1" >
		    	<b>Node number&nbsp;</b><b style="color: red;">{{ nodes[selectedNode].label }}</b>&nbsp;:&nbsp;<b>Values = <b style="color: red;" v-for="option in nodes[selectedNode].options">{{ option }}</b>.
		    </p>
		  </div>
        </form>
        <script src="https://code.jquery.com/jquery-3.3.1.slim.min.js" integrity="sha384-q8i/X+965DzO0rT7abK41JStQIAqVgRVzpbzo5smXKp4YfRvH+8abtTE1Pi6jizo" crossorigin="anonymous"></script>
		<script src="https://cdnjs.cloudflare.com/ajax/libs/popper.js/1.14.3/umd/popper.min.js" integrity="sha384-ZMP7rVo3mIykV+2+9J3UJ46jBk0WLaUAdn689aCwoqbBJiSnjAK/l8WvCWPIPm49" crossorigin="anonymous"></script>
		<script src="https://stackpath.bootstrapcdn.com/bootstrap/4.1.3/js/bootstrap.min.js" integrity="sha384-ChfqqxuZUCnJSK3+MXmPNIyE6ZbWh2IMqE241rYiqJxyMiZ6OW/JmZQ5stwEULTy" crossorigin="anonymous"></script>
  		<script src="https://cdn.jsdelivr.net/npm/vue/dist/vue.js"></script>
        <script>
        	const app = new Vue({
		  el:'#app',
		  data:{
		    nodes:[
		      {
		        label:"One",
		        options:["2, ","3, ", "4"]
		      },
		      {
		        label:"Two",
		        options:["1, ","3, ", "4"]
		      },
		      {
		        label:"Three",
		        options:["1, ","2, ", "4"]
		      },
		      {
		        label:"Four",
		        options:["1, ", "2, ", "3"]
		      }
		    ],
		    
		    selectedNode:-1,
		    selectedOption:''
		  },
		  methods:{
		    selectNode:function() {
		      this.selectedOption = '';
		    }
		  }
		});
        </script>
    "#)
}

fn main() {
	rocket::ignite().mount("/", routes![page_index]).launch();
    //println!("Hello, world!");
}
