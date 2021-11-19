using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class VehicleAnimator : MonoBehaviour
{
    public Transform From;
    public Transform To;

    public GameObject[] Cars;

    public float WaitTime;
    public float Duration;

    private void Update()
    {

        WaitTime -= Time.deltaTime;
        if (WaitTime <= 0)
        {
            // var car = Cars[Random.Range(0, Cars.Length)];
            // car.transform.position = From.position;
            //  LeanTween.move(car, To, Duration);
            WaitTime = Duration + Random.Range(5, 10);
        }
    }
}
