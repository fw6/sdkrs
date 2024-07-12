//
//  ContentView.swift
//  test
//
//  Created by Oscar on 2023-08-11.
//

import SdkrsSdk
import SwiftUI

/**
 *   IMPORTANT: This file is provided only for the purpose of demostrating the use of the SDK functionality.
 *   It hasn't gone through a throrough security review and should not be considered production ready. It also doesn't
 *   handle a lot of errors and edge cases that a production application would need to deal with.
 *   Developers are encouraged to review and improve the code as needed to meet their security requirements.
 *   Additionally, we recommend to consult with security experts and conduct thorough testing before using the code in production.
 */

struct ContentView: View {
    init() {}

    var body: some View {
        VStack {
            Button(action: {
                do {
                    print("test sdkrs")
                } catch {
                    print("ERROR:", error)
                }
            }, label: {
                Text("Test dynamic image url format")
            }).padding()

            Text("Output: " + outputText).padding(.top)
        }
        .padding()
    }
}
