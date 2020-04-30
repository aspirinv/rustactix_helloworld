import { Component } from '@angular/core';
import { HttpClient } from '@angular/common/http';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  title = 'front';
  name: string;

  constructor(private http: HttpClient){

  }

  load(){
    this.http.get<customer>("/customer/1/test").subscribe(r=>this.name = r.name)
  }
}

class customer{
  id: number;
  name: string
}
