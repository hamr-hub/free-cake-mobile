import { captureRef } from 'react-native-view-shot';
import Share from 'react-native-share';
import { RefObject } from 'react';

export async function captureAndShare(viewRef: RefObject<any>, message: string = '快来帮我投票吧！'): Promise<boolean> {
  try {
    const uri = await captureRef(viewRef, {
      format: 'png',
      quality: 0.9,
      result: 'tmpfile',
    });

    await Share.open({
      url: uri,
      message: message,
      title: '分享蛋糕设计',
    });

    return true;
  } catch (error: any) {
    if (error.message === 'User did not share') {
      return false;
    }
    throw error;
  }
}

export async function shareToWechat(url: string, title: string, description: string): Promise<boolean> {
  try {
    await Share.shareSingle({
      url,
      message: description,
      title,
      social: Share.Social.WECHAT,
    });
    return true;
  } catch {
    return false;
  }
}
