import SwiftUI

struct ContentView: View {
    var greetingService = GreetingService()
    var body: some View {
        Text(greetingService.greet("Bob"))
            .padding()
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
