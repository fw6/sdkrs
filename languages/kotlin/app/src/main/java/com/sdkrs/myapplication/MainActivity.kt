package com.sdkrs.myapplication

import android.os.Bundle
import android.util.Log
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.Button
import androidx.compose.material3.Divider
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.fragment.app.FragmentActivity
import com.sdkrs.core.NepheleImage
import com.sdkrs.myapplication.ui.theme.MyApplicationTheme
import com.sdkrs.sdk.processNepheleImageUrl

class MainActivity : FragmentActivity() {
    val TAG: String = "MainActivity"

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        setContent {
            MyApplicationTheme {
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background
                ) {
                    Column(
                        horizontalAlignment = Alignment.CenterHorizontally
                    ) {
                        Row {
                            Text(text = "Test my sdkrs sdk")
                        }

                        Divider()

                        Button(onClick = {
                            Log.i(TAG, processNepheleImageUrl(NepheleImage(
                                "https://youimg1.c-ctrip.com/target/fd/tg/g3/M07/01/AD/CggYGVXdMjCAYC76AE_wQZoMows816.jpg",
                                240u,
                            )))
                        }) {
                            Text(text = "call dynamic image url")
                        }
                    }
                }
            }
        }
    }
}
