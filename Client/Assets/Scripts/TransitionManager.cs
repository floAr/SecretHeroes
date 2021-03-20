using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class TransitionManager : MonoBehaviour
{
    public enum Location
    {
        MAIN,
        MARKET
    }

    public CameraController MainCam;
    public FadeCamera MainCamFade;

    public Transform ResetTransform;

    public Coroutine RunningTransition;

    public ClickableObject Market;

    public DrawManager DrawHall;

    public Location CurrentLocation = Location.MAIN;

    [ContextMenu("Market Transition")]
    public void TransitionIntoMarket()
    {
        if (CurrentLocation == Location.MARKET)
            return;
        StartCoroutine(MarketTransition());
    }

    public IEnumerator MarketTransition()
    {
        if (CurrentLocation != Location.MAIN)
            yield return StartCoroutine(ResetTransition());
        CurrentLocation = Location.MARKET;
        MainCam.LerpToTransform(Market.ObjectCamera.transform.position, Market.ObjectCamera.transform.rotation.eulerAngles);
        yield return new WaitForSeconds(MainCam.LerpTime - MainCamFade.Duration * 0.95f);
        MainCamFade.FadeOut();
        yield return new WaitForSeconds(MainCamFade.Duration);
        MainCam.transform.position = DrawHall.DrawCamera.transform.position;
        MainCam.transform.rotation = DrawHall.DrawCamera.transform.rotation;
        MainCamFade.FadeIn();
        yield return true;
    }

    [ContextMenu("Reset")]
    public void ResetTransitions()
    {
        CurrentLocation = Location.MAIN;
        StartCoroutine(ResetTransition());
    }

    public IEnumerator ResetTransition()
    {
        MainCamFade.FadeOut();
        yield return new WaitForSeconds(MainCamFade.Duration);
        MainCam.transform.position = ResetTransform.position;
        MainCam.transform.rotation = ResetTransform.rotation;
        MainCamFade.FadeIn();
        CurrentLocation = Location.MAIN;
        yield return true;
    }
}
