import Foundation

class GreetingService {
    func greet(_ name: String) -> String {
        let response = iosrs_greet(name)

        return String(cString: response!)
    }
}
